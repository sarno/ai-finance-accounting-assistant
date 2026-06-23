use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use uuid::Uuid;
use crate::state::AppState;
use crate::middleware::auth_middleware::AuthenticatedUser;

pub async fn audit_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    // Check if the request is a mutation (POST, PUT, DELETE) and is a normal API route
    let is_mutation = matches!(method.as_str(), "POST" | "PUT" | "DELETE");
    let is_api_route = path.starts_with("/api/") && !path.starts_with("/api/auth/");

    if !is_mutation || !is_api_route {
        return next.run(request).await;
    }

    // Extract IP address
    let ip_address = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            request
                .headers()
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.trim().to_string())
        });

    // Extract User Agent
    let user_agent = request
        .headers()
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Buffer the request body to extract the payload (e.g. for after_snapshot)
    let (req_parts, req_body) = request.into_parts();
    // Limit to 10MB to prevent memory exhaustion
    let req_bytes = match axum::body::to_bytes(req_body, 10 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => {
            tracing::error!("Failed to buffer request body in audit middleware: {:?}", e);
            return (
                StatusCode::BAD_REQUEST,
                axum::Json(serde_json::json!({
                    "errorCode": "BAD_REQUEST",
                    "message": "Failed to read request body"
                })),
            ).into_response();
        }
    };

    let req_json: Option<serde_json::Value> = serde_json::from_slice(&req_bytes).ok();
    
    // Extract the AuthenticatedUser context from the request extensions
    let user_context = req_parts
        .extensions
        .get::<AuthenticatedUser>()
        .map(|u| u.0.clone());

    // Reconstruct the request to pass to the next handler
    let request = Request::from_parts(req_parts, axum::body::Body::from(req_bytes.clone()));

    // Run the next handler
    let response = next.run(request).await;

    // Only log if request was successful (2xx or 3xx)
    let status = response.status();
    if status.is_success() {
        if let Some(user) = user_context {
            let company_id = user.company_id;
            let actor_user_id = Some(user.id);
            let actor_type = "user";

            // Infer entity type
            let entity_type = if path.contains("/journals") {
                "journal_entry"
            } else if path.contains("/sales-invoices") {
                "sales_invoice"
            } else if path.contains("/purchase-invoices") {
                "purchase_invoice"
            } else if path.contains("/payments") {
                "payment"
            } else if path.contains("/customers") {
                "customer"
            } else if path.contains("/suppliers") {
                "supplier"
            } else if path.contains("/companies") {
                "company"
            } else if path.contains("/accounts") {
                "account"
            } else if path.contains("/bank-accounts") {
                "bank_account"
            } else if path.contains("/tax-types") {
                "tax_type"
            } else if path.contains("/branches") {
                "branch"
            } else if path.contains("/item-categories") {
                "item_category"
            } else if path.contains("/items") {
                "item"
            } else if path.contains("/upload") {
                "document"
            } else {
                "unknown"
            };

            // Parse response body to check for entity id and save snapshots
            let (res_parts, res_body) = response.into_parts();
            let res_bytes = match axum::body::to_bytes(res_body, 10 * 1024 * 1024).await {
                Ok(b) => b,
                Err(e) => {
                    tracing::error!("Failed to buffer response body in audit middleware: {:?}", e);
                    return Response::from_parts(res_parts, axum::body::Body::empty());
                }
            };

            let res_json: Option<serde_json::Value> = serde_json::from_slice(&res_bytes).ok();

            // Try to find the entity_id
            let mut entity_id = None;

            // 1. Try URI path segments (from end to start)
            for segment in path.rsplit('/') {
                if let Ok(uuid) = Uuid::parse_str(segment) {
                    entity_id = Some(uuid);
                    break;
                }
            }

            // 2. Try response JSON ID
            if entity_id.is_none() {
                if let Some(ref json) = res_json {
                    if let Some(id_val) = json.get("id").and_then(|v| v.as_str()) {
                        if let Ok(uuid) = Uuid::parse_str(id_val) {
                            entity_id = Some(uuid);
                        }
                    }
                }
            }

            // 3. Try request JSON ID
            if entity_id.is_none() {
                if let Some(ref json) = req_json {
                    if let Some(id_val) = json.get("id").and_then(|v| v.as_str()) {
                        if let Ok(uuid) = Uuid::parse_str(id_val) {
                            entity_id = Some(uuid);
                        }
                    }
                }
            }

            let entity_id = entity_id.unwrap_or_else(Uuid::nil);

            // Determine Action
            let action = match method.as_str() {
                "POST" => {
                    if path.ends_with("/submit") {
                        "SUBMIT"
                    } else if path.ends_with("/approve") {
                        "APPROVE"
                    } else if path.ends_with("/reject") {
                        "REJECT"
                    } else if path.ends_with("/post") {
                        "POST"
                    } else {
                        "CREATE"
                    }
                }
                "PUT" => "UPDATE",
                "DELETE" => "DELETE",
                _ => "MUTATE",
            };

            // Set snapshots
            let before_snapshot: Option<serde_json::Value> = None;
            let after_snapshot = match method.as_str() {
                "POST" | "PUT" => req_json,
                _ => None,
            };

            // Save to database
            let db_log_res = sqlx::query!(
                r#"
                INSERT INTO audit_logs (
                    id, company_id, actor_user_id, actor_type,
                    entity_type, entity_id, action,
                    before_snapshot, after_snapshot,
                    ip_address, user_agent
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                "#,
                Uuid::new_v4(),
                company_id,
                actor_user_id,
                actor_type,
                entity_type,
                entity_id,
                action,
                before_snapshot,
                after_snapshot,
                ip_address,
                user_agent
            )
            .execute(&state.db_pool)
            .await;

            if let Err(e) = db_log_res {
                tracing::error!("Failed to write to audit_logs: {:?}", e);
            }

            return Response::from_parts(res_parts, axum::body::Body::from(res_bytes));
        }
    }

    response
}
