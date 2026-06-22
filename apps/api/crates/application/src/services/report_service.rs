use std::sync::Arc;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use rust_decimal::Decimal;
use time::Date;

use crate::errors::AppError;
use crate::dto::report::{
    CashPositionReport, CashAccountLine, ProfitLossReport, AccountBalanceLine,
    AgingReport, AgingLine, TrialBalanceReport, TrialBalanceLine,
    GeneralLedgerReport, GeneralLedgerAccountGroup, GeneralLedgerLine
};

pub struct ReportService {
    pool: PgPool,
}

impl ReportService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Calculate the cash position report as of a specific date.
    pub async fn get_cash_position(&self, company_id: Uuid, as_of: Date) -> Result<CashPositionReport, AppError> {
        let accounts_data = sqlx::query(
            "SELECT ba.bank_name, coa.name as account_name, ba.account_id
             FROM bank_accounts ba
             JOIN chart_of_accounts coa ON ba.account_id = coa.id
             WHERE ba.company_id = $1 AND ba.is_active = true"
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to fetch bank accounts: {}", e)))?;

        let mut accounts = Vec::new();
        let mut total_cash = Decimal::ZERO;

        for row in accounts_data {
            let bank_name: String = row.get("bank_name");
            let account_name: String = row.get("account_name");
            let account_id: Uuid = row.get("account_id");

            let balance_row = sqlx::query(
                "SELECT COALESCE(SUM(jl.debit - jl.credit), 0) as balance
                 FROM journal_lines jl
                 JOIN journal_entries je ON jl.journal_entry_id = je.id
                 WHERE je.company_id = $1 AND je.status = 'posted' AND jl.account_id = $2 AND je.transaction_date <= $3"
            )
            .bind(company_id)
            .bind(account_id)
            .bind(as_of)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to calculate cash balance: {}", e)))?;

            let balance: Decimal = balance_row.get("balance");
            total_cash += balance;

            accounts.push(CashAccountLine {
                bank_name,
                account_name,
                balance,
            });
        }

        Ok(CashPositionReport {
            as_of,
            total_cash,
            currency: "IDR".to_string(),
            accounts,
        })
    }

    /// Calculate the Profit & Loss report for a date range.
    pub async fn get_profit_loss(&self, company_id: Uuid, period_from: Date, period_to: Date) -> Result<ProfitLossReport, AppError> {
        let rows = sqlx::query(
            "SELECT coa.id as account_id, coa.code as account_code, coa.name as account_name, coa.account_type,
                    COALESCE(SUM(jl.debit), 0) as total_debit,
                    COALESCE(SUM(jl.credit), 0) as total_credit
             FROM chart_of_accounts coa
             LEFT JOIN journal_lines jl ON coa.id = jl.account_id
             LEFT JOIN journal_entries je ON jl.journal_entry_id = je.id AND je.status = 'posted' AND je.transaction_date >= $2 AND je.transaction_date <= $3
             WHERE coa.company_id = $1
             GROUP BY coa.id, coa.code, coa.name, coa.account_type
             ORDER BY coa.code"
        )
        .bind(company_id)
        .bind(period_from)
        .bind(period_to)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to query profit and loss: {}", e)))?;

        let mut revenue_accounts = Vec::new();
        let mut expense_accounts = Vec::new();
        let mut total_revenue = Decimal::ZERO;
        let mut total_expense = Decimal::ZERO;

        for row in rows {
            let account_type: String = row.get("account_type");
            let total_debit: Decimal = row.get("total_debit");
            let total_credit: Decimal = row.get("total_credit");

            let account_id: Uuid = row.get("account_id");
            let account_code: String = row.get("account_code");
            let account_name: String = row.get("account_name");

            if account_type.eq_ignore_ascii_case("Revenue") {
                let balance = total_credit - total_debit;
                if balance != Decimal::ZERO {
                    total_revenue += balance;
                    revenue_accounts.push(AccountBalanceLine {
                        account_id,
                        account_code,
                        account_name,
                        balance,
                    });
                }
            } else if account_type.eq_ignore_ascii_case("Expense") {
                let balance = total_debit - total_credit;
                if balance != Decimal::ZERO {
                    total_expense += balance;
                    expense_accounts.push(AccountBalanceLine {
                        account_id,
                        account_code,
                        account_name,
                        balance,
                    });
                }
            }
        }

        let net_profit = total_revenue - total_expense;

        Ok(ProfitLossReport {
            period_from,
            period_to,
            revenue_accounts,
            total_revenue,
            expense_accounts,
            total_expense,
            net_profit,
        })
    }

    /// Calculate the Accounts Receivable aging report.
    pub async fn get_accounts_receivable_aging(&self, company_id: Uuid, as_of: Date) -> Result<AgingReport, AppError> {
        let invoices = sqlx::query(
            "SELECT si.id, si.customer_id, cust.name as customer_name, si.invoice_date, si.due_date, si.total_amount
             FROM sales_invoices si
             JOIN customers cust ON si.customer_id = cust.id
             WHERE si.company_id = $1 AND si.status = 'posted' AND si.invoice_date <= $2"
        )
        .bind(company_id)
        .bind(as_of)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to fetch A/R invoices: {}", e)))?;

        use std::collections::HashMap;

        #[derive(Default)]
        struct CustomerAging {
            name: String,
            total_outstanding: Decimal,
            current: Decimal,
            days_1_30: Decimal,
            days_31_60: Decimal,
            days_61_90: Decimal,
            days_90_plus: Decimal,
        }

        let mut customer_map: HashMap<Uuid, CustomerAging> = HashMap::new();
        let mut total_outstanding = Decimal::ZERO;

        for row in invoices {
            let invoice_id: Uuid = row.get("id");
            let customer_id: Uuid = row.get("customer_id");
            let customer_name: String = row.get("customer_name");
            let due_date: Date = row.get("due_date");
            let total_amount: Decimal = row.get("total_amount");

            let paid_row = sqlx::query(
                "SELECT COALESCE(SUM(pa.allocated_amount), 0) as paid_amount
                 FROM payment_allocations pa
                 JOIN payments p ON pa.payment_id = p.id
                 WHERE pa.document_type = 'SalesInvoice' AND pa.document_id = $1 AND p.status = 'posted' AND p.payment_date <= $2"
            )
            .bind(invoice_id)
            .bind(as_of)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to fetch paid amount for sales invoice: {}", e)))?;

            let paid_amount: Decimal = paid_row.get("paid_amount");
            let outstanding = total_amount - paid_amount;

            if outstanding <= Decimal::ZERO {
                continue;
            }

            total_outstanding += outstanding;

            let entry = customer_map.entry(customer_id).or_insert_with(|| CustomerAging {
                name: customer_name,
                ..Default::default()
            });

            entry.total_outstanding += outstanding;

            if as_of <= due_date {
                entry.current += outstanding;
            } else {
                let overdue_days = as_of.to_julian_day() - due_date.to_julian_day();
                if overdue_days <= 30 {
                    entry.days_1_30 += outstanding;
                } else if overdue_days <= 60 {
                    entry.days_31_60 += outstanding;
                } else if overdue_days <= 90 {
                    entry.days_61_90 += outstanding;
                } else {
                    entry.days_90_plus += outstanding;
                }
            }
        }

        let mut lines = Vec::new();
        for (cust_id, aging) in customer_map {
            lines.push(AgingLine {
                counterparty_id: Some(cust_id),
                counterparty_name: aging.name,
                total_outstanding: aging.total_outstanding,
                current: aging.current,
                days_1_30: aging.days_1_30,
                days_31_60: aging.days_31_60,
                days_61_90: aging.days_61_90,
                days_90_plus: aging.days_90_plus,
            });
        }

        lines.sort_by(|a, b| a.counterparty_name.cmp(&b.counterparty_name));

        Ok(AgingReport {
            as_of,
            total_outstanding,
            lines,
        })
    }

    /// Calculate the Accounts Payable aging report.
    pub async fn get_accounts_payable_aging(&self, company_id: Uuid, as_of: Date) -> Result<AgingReport, AppError> {
        let invoices = sqlx::query(
            "SELECT pi.id, pi.supplier_id, sup.name as supplier_name, pi.invoice_date, pi.due_date, pi.total_amount
             FROM purchase_invoices pi
             JOIN suppliers sup ON pi.supplier_id = sup.id
             WHERE pi.company_id = $1 AND pi.status = 'posted' AND pi.invoice_date <= $2"
        )
        .bind(company_id)
        .bind(as_of)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to fetch A/P invoices: {}", e)))?;

        use std::collections::HashMap;

        #[derive(Default)]
        struct SupplierAging {
            name: String,
            total_outstanding: Decimal,
            current: Decimal,
            days_1_30: Decimal,
            days_31_60: Decimal,
            days_61_90: Decimal,
            days_90_plus: Decimal,
        }

        let mut supplier_map: HashMap<Uuid, SupplierAging> = HashMap::new();
        let mut total_outstanding = Decimal::ZERO;

        for row in invoices {
            let invoice_id: Uuid = row.get("id");
            let supplier_id: Uuid = row.get("supplier_id");
            let supplier_name: String = row.get("supplier_name");
            let due_date: Date = row.get("due_date");
            let total_amount: Decimal = row.get("total_amount");

            let paid_row = sqlx::query(
                "SELECT COALESCE(SUM(pa.allocated_amount), 0) as paid_amount
                 FROM payment_allocations pa
                 JOIN payments p ON pa.payment_id = p.id
                 WHERE pa.document_type = 'PurchaseInvoice' AND pa.document_id = $1 AND p.status = 'posted' AND p.payment_date <= $2"
            )
            .bind(invoice_id)
            .bind(as_of)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to fetch paid amount for purchase invoice: {}", e)))?;

            let paid_amount: Decimal = paid_row.get("paid_amount");
            let outstanding = total_amount - paid_amount;

            if outstanding <= Decimal::ZERO {
                continue;
            }

            total_outstanding += outstanding;

            let entry = supplier_map.entry(supplier_id).or_insert_with(|| SupplierAging {
                name: supplier_name,
                ..Default::default()
            });

            entry.total_outstanding += outstanding;

            if as_of <= due_date {
                entry.current += outstanding;
            } else {
                let overdue_days = as_of.to_julian_day() - due_date.to_julian_day();
                if overdue_days <= 30 {
                    entry.days_1_30 += outstanding;
                } else if overdue_days <= 60 {
                    entry.days_31_60 += outstanding;
                } else if overdue_days <= 90 {
                    entry.days_61_90 += outstanding;
                } else {
                    entry.days_90_plus += outstanding;
                }
            }
        }

        let mut lines = Vec::new();
        for (sup_id, aging) in supplier_map {
            lines.push(AgingLine {
                counterparty_id: Some(sup_id),
                counterparty_name: aging.name,
                total_outstanding: aging.total_outstanding,
                current: aging.current,
                days_1_30: aging.days_1_30,
                days_31_60: aging.days_31_60,
                days_61_90: aging.days_61_90,
                days_90_plus: aging.days_90_plus,
            });
        }

        lines.sort_by(|a, b| a.counterparty_name.cmp(&b.counterparty_name));

        Ok(AgingReport {
            as_of,
            total_outstanding,
            lines,
        })
    }

    /// Calculate the Trial Balance report.
    pub async fn get_trial_balance(&self, company_id: Uuid, as_of: Date) -> Result<TrialBalanceReport, AppError> {
        let rows = sqlx::query(
            "SELECT coa.id as account_id, coa.code as account_code, coa.name as account_name,
                    COALESCE(SUM(jl.debit), 0) as total_debit,
                    COALESCE(SUM(jl.credit), 0) as total_credit
             FROM chart_of_accounts coa
             LEFT JOIN journal_lines jl ON coa.id = jl.account_id
             LEFT JOIN journal_entries je ON jl.journal_entry_id = je.id AND je.status = 'posted' AND je.transaction_date <= $2
             WHERE coa.company_id = $1
             GROUP BY coa.id, coa.code, coa.name
             ORDER BY coa.code"
        )
        .bind(company_id)
        .bind(as_of)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to query trial balance: {}", e)))?;

        let mut lines = Vec::new();
        let mut total_debit = Decimal::ZERO;
        let mut total_credit = Decimal::ZERO;

        for row in rows {
            let account_id: Uuid = row.get("account_id");
            let account_code: String = row.get("account_code");
            let account_name: String = row.get("account_name");
            let debit: Decimal = row.get("total_debit");
            let credit: Decimal = row.get("total_credit");

            let mut debit_balance = Decimal::ZERO;
            let mut credit_balance = Decimal::ZERO;

            if debit >= credit {
                debit_balance = debit - credit;
            } else {
                credit_balance = credit - debit;
            }

            if debit_balance != Decimal::ZERO || credit_balance != Decimal::ZERO {
                total_debit += debit_balance;
                total_credit += credit_balance;

                lines.push(TrialBalanceLine {
                    account_id,
                    account_code,
                    account_name,
                    debit_balance,
                    credit_balance,
                });
            }
        }

        Ok(TrialBalanceReport {
            as_of,
            total_debit,
            total_credit,
            lines,
        })
    }

    /// Calculate the General Ledger transaction records.
    pub async fn get_general_ledger(&self, company_id: Uuid, start_date: Date, end_date: Date, filter_account_id: Option<Uuid>) -> Result<GeneralLedgerReport, AppError> {
        let accounts_query = if let Some(aid) = filter_account_id {
            sqlx::query(
                "SELECT id as account_id, code as account_code, name as account_name, account_type
                 FROM chart_of_accounts
                 WHERE company_id = $1 AND id = $2 AND is_active = true"
            )
            .bind(company_id)
            .bind(aid)
        } else {
            sqlx::query(
                "SELECT id as account_id, code as account_code, name as account_name, account_type
                 FROM chart_of_accounts
                 WHERE company_id = $1 AND is_active = true
                 ORDER BY code"
            )
            .bind(company_id)
        };

        let accounts_data = accounts_query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to fetch chart of accounts: {}", e)))?;

        let mut accounts = Vec::new();

        for row in accounts_data {
            let account_id: Uuid = row.get("account_id");
            let account_code: String = row.get("account_code");
            let account_name: String = row.get("account_name");
            let account_type: String = row.get("account_type");

            let is_asset_or_expense = account_type.eq_ignore_ascii_case("Asset") || account_type.eq_ignore_ascii_case("Expense");

            let open_bal_row = sqlx::query(
                "SELECT COALESCE(SUM(jl.debit), 0) as open_debit, COALESCE(SUM(jl.credit), 0) as open_credit
                 FROM journal_lines jl
                 JOIN journal_entries je ON jl.journal_entry_id = je.id
                 WHERE je.company_id = $1 AND je.status = 'posted' AND jl.account_id = $2 AND je.transaction_date < $3"
            )
            .bind(company_id)
            .bind(account_id)
            .bind(start_date)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to calculate opening balance: {}", e)))?;

            let open_debit: Decimal = open_bal_row.get("open_debit");
            let open_credit: Decimal = open_bal_row.get("open_credit");

            let opening_balance = if is_asset_or_expense {
                open_debit - open_credit
            } else {
                open_credit - open_debit
            };

            let transactions = sqlx::query(
                "SELECT je.transaction_date, je.reference_number, je.description, jl.debit, jl.credit
                 FROM journal_lines jl
                 JOIN journal_entries je ON jl.journal_entry_id = je.id
                 WHERE je.company_id = $1 AND je.status = 'posted' AND jl.account_id = $2 AND je.transaction_date >= $3 AND je.transaction_date <= $4
                 ORDER BY je.transaction_date, je.reference_number, jl.id"
            )
            .bind(company_id)
            .bind(account_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to fetch ledger transactions: {}", e)))?;

            let mut lines = Vec::new();
            let mut running_balance = opening_balance;

            for t in transactions {
                let transaction_date: Date = t.get("transaction_date");
                let reference_number: String = t.get("reference_number");
                let description: String = t.get("description");
                let debit: Decimal = t.get("debit");
                let credit: Decimal = t.get("credit");

                if is_asset_or_expense {
                    running_balance += debit - credit;
                } else {
                    running_balance += credit - debit;
                }

                lines.push(GeneralLedgerLine {
                    transaction_date,
                    reference_number,
                    description,
                    debit,
                    credit,
                    running_balance,
                });
            }

            accounts.push(GeneralLedgerAccountGroup {
                account_id,
                account_code,
                account_name,
                opening_balance,
                closing_balance: running_balance,
                lines,
            });
        }

        Ok(GeneralLedgerReport {
            start_date,
            end_date,
            accounts,
        })
    }
}



