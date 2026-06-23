use sqlx::{PgPool, Row};
use std::time::Duration;
use std::fs;
use std::path::Path;
use time::OffsetDateTime;

fn rc4_encrypt_decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut s: Vec<u8> = (0..=255).collect();
    let mut j: usize = 0;
    for i in 0..256 {
        j = (j + s[i] as usize + key[i % key.len()] as usize) % 256;
        s.swap(i, j);
    }
    
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut out = Vec::with_capacity(data.len());
    for &byte in data {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        let k = s[(s[i] as usize + s[j] as usize) % 256];
        out.push(byte ^ k);
    }
    out
}

pub async fn run(pool: PgPool) -> anyhow::Result<()> {
    // Run backup job every hour
    let mut interval = tokio::time::interval(Duration::from_secs(3600));
    loop {
        interval.tick().await;
        tracing::info!("Running database backup job...");

        match perform_backup(&pool).await {
            Ok(path) => tracing::info!("Database backup completed successfully: {}", path),
            Err(e) => tracing::error!("Database backup failed: {:?}", e),
        }
    }
}

async fn perform_backup(pool: &PgPool) -> anyhow::Result<String> {
    let tables = vec![
        "companies", "users", "branches", "accounts", "customers", 
        "suppliers", "bank_accounts", "tax_types", "tax_records", 
        "tax_calendar", "journal_entries", "journal_lines", "sales_invoices", 
        "sales_invoice_item_links", "purchase_invoices", "purchase_invoice_lines", 
        "payments", "uploaded_documents", "audit_logs", "idempotency_keys"
    ];

    let mut backup_data = serde_json::Map::new();
    for table in tables {
        let query_str = format!("SELECT coalesce(json_agg(t), '[]'::json) as data FROM {} t", table);
        let row = sqlx::query(&query_str).fetch_one(pool).await?;
        let data: serde_json::Value = row.try_get("data")?;
        backup_data.insert(table.to_string(), data);
    }

    let json_bytes = serde_json::to_vec(&backup_data)?;

    // Encryption
    let encryption_key = std::env::var("BACKUP_ENCRYPTION_KEY")
        .unwrap_or_else(|_| "finance_secret_backup_key_12345".to_string());
    
    let encrypted_bytes = rc4_encrypt_decrypt(encryption_key.as_bytes(), &json_bytes);

    // Write to ./target/backups/
    let backups_dir = Path::new("./target/backups");
    fs::create_dir_all(backups_dir)?;

    let timestamp = OffsetDateTime::now_utc().unix_timestamp();
    let backup_filename = format!("backup_{}.enc", timestamp);
    let backup_path = backups_dir.join(&backup_filename);

    fs::write(&backup_path, encrypted_bytes)?;

    // Clean up old backups (keep latest 5)
    if let Ok(entries) = fs::read_dir(backups_dir) {
        let mut files = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("enc") {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        files.push((path, modified));
                    }
                }
            }
        }
        
        // Sort files by modified time (oldest first)
        files.sort_by_key(|x| x.1);

        if files.len() > 5 {
            let to_delete = files.len() - 5;
            for i in 0..to_delete {
                let _ = fs::remove_file(&files[i].0);
            }
        }
    }

    Ok(backup_path.to_string_lossy().to_string())
}
