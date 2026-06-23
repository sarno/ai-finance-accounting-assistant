use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use crate::state::AppState;

pub async fn idempotency_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let key = match request
        .headers()
        .get("x-idempotency-key")
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.trim().is_empty())
    {
        Some(k) => k.to_string(),
        None => return next.run(request).await,
    };

    // Check if key already exists
    let existing = sqlx::query!(
        "SELECT response_status, response_body FROM idempotency_keys WHERE key = $1",
        key
    )
    .fetch_optional(&state.db_pool)
    .await;

    match existing {
        Ok(Some(row)) => {
            tracing::info!("Idempotent hit for key: {}", key);
            return Response::builder()
                .status(StatusCode::from_u16(row.response_status as u16).unwrap_or(StatusCode::OK))
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(row.response_body))
                .unwrap()
                .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to check idempotency key: {:?}", e);
            // Fallback to calling the service directly if database fails to query
        }
        _ => {}
    }

    // Process the request
    let response = next.run(request).await;

    // We only cache successful/client error responses (status < 500).
    // Avoid caching server errors so that the client can retry safely.
    let status = response.status();
    if status.as_u16() < 500 {
        let (parts, body) = response.into_parts();
        // Limit to 10MB to prevent memory exhaustion
        match axum::body::to_bytes(body, 10 * 1024 * 1024).await {
            Ok(bytes) => {
                let body_str = String::from_utf8_lossy(&bytes).into_owned();
                
                let save_result = sqlx::query!(
                    "INSERT INTO idempotency_keys (key, response_status, response_body) VALUES ($1, $2, $3) ON CONFLICT (key) DO NOTHING",
                    key,
                    status.as_u16() as i32,
                    body_str
                )
                .execute(&state.db_pool)
                .await;

                if let Err(e) = save_result {
                    tracing::error!("Failed to save idempotency key response: {:?}", e);
                }

                return Response::from_parts(parts, axum::body::Body::from(bytes));
            }
            Err(e) => {
                tracing::error!("Failed to read response body for caching: {:?}", e);
                return Response::from_parts(parts, axum::body::Body::empty());
            }
        }
    }

    response
}
