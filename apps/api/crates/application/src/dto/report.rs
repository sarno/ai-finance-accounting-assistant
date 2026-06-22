use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CashPositionReport {
    #[serde(with = "crate::dto::date_format")]
    pub as_of: time::Date,
    pub total_cash: Decimal,
    pub currency: String,
    pub accounts: Vec<CashAccountLine>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CashAccountLine {
    pub bank_name: String,
    pub account_name: String,
    pub balance: Decimal,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfitLossReport {
    #[serde(with = "crate::dto::date_format")]
    pub period_from: time::Date,
    #[serde(with = "crate::dto::date_format")]
    pub period_to: time::Date,
    pub revenue_accounts: Vec<AccountBalanceLine>,
    pub total_revenue: Decimal,
    pub expense_accounts: Vec<AccountBalanceLine>,
    pub total_expense: Decimal,
    pub net_profit: Decimal,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalanceLine {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub balance: Decimal,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgingReport {
    #[serde(with = "crate::dto::date_format")]
    pub as_of: time::Date,
    pub total_outstanding: Decimal,
    pub lines: Vec<AgingLine>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgingLine {
    pub counterparty_id: Option<Uuid>,
    pub counterparty_name: String,
    pub total_outstanding: Decimal,
    pub current: Decimal,
    pub days_1_30: Decimal,
    pub days_31_60: Decimal,
    pub days_61_90: Decimal,
    pub days_90_plus: Decimal,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrialBalanceReport {
    #[serde(with = "crate::dto::date_format")]
    pub as_of: time::Date,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub lines: Vec<TrialBalanceLine>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrialBalanceLine {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub debit_balance: Decimal,
    pub credit_balance: Decimal,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralLedgerReport {
    #[serde(with = "crate::dto::date_format")]
    pub start_date: time::Date,
    #[serde(with = "crate::dto::date_format")]
    pub end_date: time::Date,
    pub accounts: Vec<GeneralLedgerAccountGroup>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralLedgerAccountGroup {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub opening_balance: Decimal,
    pub closing_balance: Decimal,
    pub lines: Vec<GeneralLedgerLine>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralLedgerLine {
    #[serde(with = "crate::dto::date_format")]
    pub transaction_date: time::Date,
    pub reference_number: String,
    pub description: String,
    pub debit: Decimal,
    pub credit: Decimal,
    pub running_balance: Decimal,
}

