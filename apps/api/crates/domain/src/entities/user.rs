use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub company_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub password_hash: String,
    pub roles: Vec<UserRole>,
    pub is_active: bool,
    pub last_login_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Owner,
    FinanceManager,
    AccountingStaff,
    TaxStaff,
    Auditor,
    Admin,
    AiService,
}

impl UserRole {
    pub fn can_approve(&self) -> bool {
        matches!(self, UserRole::Owner | UserRole::FinanceManager | UserRole::Admin)
    }

    pub fn can_post(&self) -> bool {
        matches!(self, UserRole::Owner | UserRole::FinanceManager | UserRole::AccountingStaff | UserRole::Admin)
    }

    pub fn can_configure_tax(&self) -> bool {
        matches!(self, UserRole::Owner | UserRole::Admin | UserRole::TaxStaff)
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UserRole::Owner => "owner",
            UserRole::FinanceManager => "finance_manager",
            UserRole::AccountingStaff => "accounting_staff",
            UserRole::TaxStaff => "tax_staff",
            UserRole::Auditor => "auditor",
            UserRole::Admin => "admin",
            UserRole::AiService => "ai_service",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "owner" => Ok(UserRole::Owner),
            "finance_manager" | "financemanager" => Ok(UserRole::FinanceManager),
            "accounting_staff" | "accountingstaff" => Ok(UserRole::AccountingStaff),
            "tax_staff" | "taxstaff" => Ok(UserRole::TaxStaff),
            "auditor" => Ok(UserRole::Auditor),
            "admin" => Ok(UserRole::Admin),
            "ai_service" | "aiservice" => Ok(UserRole::AiService),
            other => Err(format!("Unknown user role: {}", other)),
        }
    }
}
