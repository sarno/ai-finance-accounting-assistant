use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Debug, Serialize)]
pub struct CashPositionReport {
    pub as_of: time::Date,
    pub total_cash: Decimal,
    pub currency: String,
    pub accounts: Vec<CashAccountLine>,
}

#[derive(Debug, Serialize)]
pub struct CashAccountLine {
    pub bank_name: String,
    pub account_name: String,
    pub balance: Decimal,
}

#[derive(Debug, Serialize)]
pub struct ProfitLossReport {
    pub period_from: time::Date,
    pub period_to: time::Date,
    pub total_revenue: Decimal,
    pub total_expense: Decimal,
    pub net_profit: Decimal,
}
