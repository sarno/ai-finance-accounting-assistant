use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{
    errors::AppError,
    ports::tax_repository::TaxRepository,
};
use finance_assistant_domain::entities::tax::{TaxCategory, TaxType};

pub struct PgTaxRepository {
    pool: PgPool,
}

impl PgTaxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn map_category_to_str(cat: &TaxCategory) -> &'static str {
    match cat {
        TaxCategory::VatOutput => "vat_output",
        TaxCategory::VatInput => "vat_input",
        TaxCategory::WithholdingPph21 => "withholding_pph21",
        TaxCategory::WithholdingPph23 => "withholding_pph23",
        TaxCategory::WithholdingPph25 => "withholding_pph25",
        TaxCategory::WithholdingPphFinal => "withholding_pph_final",
    }
}

fn map_str_to_category(s: &str) -> TaxCategory {
    match s.to_lowercase().as_str() {
        "vat_output" | "vatoutput" => TaxCategory::VatOutput,
        "vat_input" | "vatinput" => TaxCategory::VatInput,
        "withholding_pph21" | "withholdingpph21" => TaxCategory::WithholdingPph21,
        "withholding_pph23" | "withholdingpph23" => TaxCategory::WithholdingPph23,
        "withholding_pph25" | "withholdingpph25" => TaxCategory::WithholdingPph25,
        "withholding_pph_final" | "withholdingpphfinal" => TaxCategory::WithholdingPphFinal,
        _ => TaxCategory::VatOutput,
    }
}

#[async_trait]
impl TaxRepository for PgTaxRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<TaxType, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, code, name, category, default_rate, payable_account_id, effective_from, effective_to, is_active, created_at, updated_at
            FROM tax_types
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let row = match row {
            Some(r) => r,
            None => return Err(AppError::NotFound {
                resource: "TaxType".to_string(),
                id: id.to_string(),
            }),
        };

        let cat_str: String = row.get("category");

        Ok(TaxType {
            id: row.get("id"),
            company_id: row.get("company_id"),
            code: row.get("code"),
            name: row.get("name"),
            category: map_str_to_category(&cat_str),
            default_rate: row.get("default_rate"),
            payable_account_id: row.get("payable_account_id"),
            effective_from: row.get("effective_from"),
            effective_to: row.get("effective_to"),
            is_active: row.get("is_active"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<TaxType>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, code, name, category, default_rate, payable_account_id, effective_from, effective_to, is_active, created_at, updated_at
            FROM tax_types
            WHERE company_id = $1
            ORDER BY code ASC
            "#,
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let tax_types = rows
            .into_iter()
            .map(|row| {
                let cat_str: String = row.get("category");
                TaxType {
                    id: row.get("id"),
                    company_id: row.get("company_id"),
                    code: row.get("code"),
                    name: row.get("name"),
                    category: map_str_to_category(&cat_str),
                    default_rate: row.get("default_rate"),
                    payable_account_id: row.get("payable_account_id"),
                    effective_from: row.get("effective_from"),
                    effective_to: row.get("effective_to"),
                    is_active: row.get("is_active"),
                    created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                    updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
                }
            })
            .collect();

        Ok(tax_types)
    }

    async fn save(&self, tax_type: &TaxType) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO tax_types (id, company_id, code, name, category, default_rate, payable_account_id, effective_from, effective_to, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(tax_type.id)
        .bind(tax_type.company_id)
        .bind(&tax_type.code)
        .bind(&tax_type.name)
        .bind(map_category_to_str(&tax_type.category))
        .bind(tax_type.default_rate)
        .bind(tax_type.payable_account_id)
        .bind(tax_type.effective_from)
        .bind(tax_type.effective_to)
        .bind(tax_type.is_active)
        .bind(tax_type.created_at)
        .bind(tax_type.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update(&self, tax_type: &TaxType) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE tax_types
            SET name = $2, category = $3, default_rate = $4, payable_account_id = $5, effective_from = $6, effective_to = $7, is_active = $8, updated_at = $9
            WHERE id = $1
            "#,
        )
        .bind(tax_type.id)
        .bind(&tax_type.name)
        .bind(map_category_to_str(&tax_type.category))
        .bind(tax_type.default_rate)
        .bind(tax_type.payable_account_id)
        .bind(tax_type.effective_from)
        .bind(tax_type.effective_to)
        .bind(tax_type.is_active)
        .bind(tax_type.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}
