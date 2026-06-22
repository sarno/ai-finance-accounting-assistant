use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{errors::ApiError, middleware::auth_middleware::AuthenticatedUser, state::AppState};
use finance_assistant_app::dto::report::{
    CashPositionReport, ProfitLossReport, AgingReport, TrialBalanceReport, GeneralLedgerReport
};
use finance_assistant_app::dto::master_data::TaxSummaryResponse;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashPositionParams {
    pub as_of: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfitLossParams {
    pub period_from: Option<String>,
    pub period_to: Option<String>,
    pub period: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgingParams {
    pub as_of: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrialBalanceParams {
    pub as_of: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralLedgerParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub account_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxSummaryParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub period: Option<String>,
}

// Helper function to parse Date from query params supporting RFC3339 or YYYY-MM-DD
fn parse_date(date_str: &Option<String>, default: time::Date) -> Result<time::Date, ApiError> {
    match date_str {
        Some(s) if !s.is_empty() => {
            time::Date::parse(s, &time::format_description::well_known::Rfc3339)
                .or_else(|_| time::Date::parse(s, &time::macros::format_description!("[year]-[month]-[day]")))
                .map_err(|e| ApiError(finance_assistant_app::errors::AppError::Validation {
                    message: format!("Invalid date '{}': {}", s, e),
                }))
        }
        _ => Ok(default)
    }
}

/// GET /api/reports/cash-position
pub async fn cash_position(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<CashPositionParams>,
) -> Result<Json<CashPositionReport>, ApiError> {
    let as_of = parse_date(&params.as_of, time::OffsetDateTime::now_utc().date())?;
    let report = state.report_service.get_cash_position(user.company_id, as_of).await?;
    Ok(Json(report))
}

/// GET /api/reports/profit-loss
pub async fn profit_loss(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<ProfitLossParams>,
) -> Result<Json<ProfitLossReport>, ApiError> {
    let now = time::OffsetDateTime::now_utc().date();
    let mut from_date = time::Date::from_calendar_date(now.year(), now.month(), 1).unwrap();
    let mut to_date = now;

    if let Some(ref p) = params.period {
        let parts: Vec<&str> = p.split('-').collect();
        if parts.len() == 2 {
            if let (Ok(y), Ok(m)) = (parts[0].parse::<i32>(), parts[1].parse::<u8>()) {
                if let Ok(month) = time::Month::try_from(m) {
                    if let Ok(start) = time::Date::from_calendar_date(y, month, 1) {
                        from_date = start;
                        let next_month = month.next();
                        let next_year = if next_month == time::Month::January { y + 1 } else { y };
                        let next_month_start = time::Date::from_calendar_date(next_year, next_month, 1).unwrap();
                        to_date = next_month_start.previous_day().unwrap();
                    }
                }
            }
        }
    } else {
        from_date = parse_date(&params.period_from, from_date)?;
        to_date = parse_date(&params.period_to, to_date)?;
    }

    let report = state.report_service.get_profit_loss(user.company_id, from_date, to_date).await?;
    Ok(Json(report))
}

/// GET /api/reports/accounts-receivable
pub async fn accounts_receivable(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<AgingParams>,
) -> Result<Json<AgingReport>, ApiError> {
    let as_of = parse_date(&params.as_of, time::OffsetDateTime::now_utc().date())?;
    let report = state.report_service.get_accounts_receivable_aging(user.company_id, as_of).await?;
    Ok(Json(report))
}

/// GET /api/reports/accounts-payable
pub async fn accounts_payable(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<AgingParams>,
) -> Result<Json<AgingReport>, ApiError> {
    let as_of = parse_date(&params.as_of, time::OffsetDateTime::now_utc().date())?;
    let report = state.report_service.get_accounts_payable_aging(user.company_id, as_of).await?;
    Ok(Json(report))
}

/// GET /api/reports/trial-balance
pub async fn trial_balance(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<TrialBalanceParams>,
) -> Result<Json<TrialBalanceReport>, ApiError> {
    let as_of = parse_date(&params.as_of, time::OffsetDateTime::now_utc().date())?;
    let report = state.report_service.get_trial_balance(user.company_id, as_of).await?;
    Ok(Json(report))
}

/// GET /api/reports/general-ledger
pub async fn general_ledger(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<GeneralLedgerParams>,
) -> Result<Json<GeneralLedgerReport>, ApiError> {
    let now = time::OffsetDateTime::now_utc().date();
    let start_date = time::Date::from_calendar_date(now.year(), now.month(), 1).unwrap();
    
    let start = parse_date(&params.start_date, start_date)?;
    let end = parse_date(&params.end_date, now)?;

    let report = state.report_service.get_general_ledger(user.company_id, start, end, params.account_id).await?;
    Ok(Json(report))
}

/// GET /api/reports/tax-summary
pub async fn tax_summary(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<TaxSummaryParams>,
) -> Result<Json<TaxSummaryResponse>, ApiError> {
    let now = time::OffsetDateTime::now_utc().date();
    let mut from_date = time::Date::from_calendar_date(now.year(), now.month(), 1).unwrap();
    let mut to_date = now;

    if let Some(ref p) = params.period {
        let parts: Vec<&str> = p.split('-').collect();
        if parts.len() == 2 {
            if let (Ok(y), Ok(m)) = (parts[0].parse::<i32>(), parts[1].parse::<u8>()) {
                if let Ok(month) = time::Month::try_from(m) {
                    if let Ok(start) = time::Date::from_calendar_date(y, month, 1) {
                        from_date = start;
                        let next_month = month.next();
                        let next_year = if next_month == time::Month::January { y + 1 } else { y };
                        let next_month_start = time::Date::from_calendar_date(next_year, next_month, 1).unwrap();
                        to_date = next_month_start.previous_day().unwrap();
                    }
                }
            }
        }
    } else {
        from_date = parse_date(&params.start_date, from_date)?;
        to_date = parse_date(&params.end_date, to_date)?;
    }

    let report = state.master_data_service.get_tax_summary(user.company_id, from_date, to_date).await?;
    Ok(Json(report))
}
