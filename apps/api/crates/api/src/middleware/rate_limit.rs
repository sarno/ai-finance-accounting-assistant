use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::{Arc, OnceLock},
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

pub struct RateLimiter {
    requests: Mutex<HashMap<IpAddr, Vec<Instant>>>,
    limit: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(limit: usize, window: Duration) -> Self {
        Self {
            requests: Mutex::new(HashMap::new()),
            limit,
            window,
        }
    }

    pub async fn check_rate_limit(&self, ip: IpAddr) -> bool {
        let mut map = self.requests.lock().await;
        let now = Instant::now();
        let list = map.entry(ip).or_default();

        // Remove timestamps outside the window
        list.retain(|&time| now.duration_since(time) < self.window);

        if list.len() >= self.limit {
            false
        } else {
            list.push(now);
            true
        }
    }
}

pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Response {
    // Determine client IP address
    let client_ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse::<IpAddr>().ok())
        .or_else(|| {
            request
                .headers()
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.trim().parse::<IpAddr>().ok())
        })
        .unwrap_or_else(|| "127.0.0.1".parse().unwrap());

    static LIMITER: OnceLock<RateLimiter> = OnceLock::new();
    let limiter = LIMITER.get_or_init(|| RateLimiter::new(100, Duration::from_secs(60)));

    if !limiter.check_rate_limit(client_ip).await {
        tracing::warn!("Rate limit exceeded for IP: {}", client_ip);
        return (
            StatusCode::TOO_MANY_REQUESTS,
            axum::Json(serde_json::json!({
                "errorCode": "RATE_LIMIT_EXCEEDED",
                "message": "Rate limit exceeded. Please try again later."
            })),
        )
            .into_response();
    }

    next.run(request).await
}
