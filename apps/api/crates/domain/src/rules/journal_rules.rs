use rust_decimal::Decimal;

use crate::{entities::journal::JournalLine, errors::DomainError};

/// Ensure all journal lines sum to zero (debit == credit).
pub fn validate_journal_balance(lines: &[JournalLine]) -> Result<(), DomainError> {
    let total_debit: Decimal = lines.iter().map(|l| l.debit).sum();
    let total_credit: Decimal = lines.iter().map(|l| l.credit).sum();

    if total_debit != total_credit {
        return Err(DomainError::JournalNotBalanced { total_debit, total_credit });
    }
    Ok(())
}

/// Ensure each line has debit XOR credit, not both.
pub fn validate_journal_line_sides(lines: &[JournalLine]) -> Result<(), DomainError> {
    for line in lines {
        if line.debit > Decimal::ZERO && line.credit > Decimal::ZERO {
            return Err(DomainError::Validation {
                message: "Journal line cannot have both debit and credit".to_string(),
            });
        }
        if line.debit == Decimal::ZERO && line.credit == Decimal::ZERO {
            return Err(DomainError::Validation {
                message: "Journal line must have either debit or credit".to_string(),
            });
        }
    }
    Ok(())
}
