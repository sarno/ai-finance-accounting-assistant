use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::str::FromStr;

use finance_assistant_app::{
    errors::AppError,
    ports::user_repository::UserRepository,
};
use finance_assistant_domain::entities::user::{User, UserRole};

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
        let user_row = sqlx::query(
            r#"
            SELECT id, company_id, email, full_name, password_hash, is_active, last_login_at, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let user_row = match user_row {
            Some(row) => row,
            None => return Err(AppError::NotFound {
                resource: "User".to_string(),
                id: id.to_string(),
            }),
        };

        let role_rows = sqlx::query(
            r#"
            SELECT role
            FROM user_roles
            WHERE user_id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let roles: Vec<UserRole> = role_rows
            .into_iter()
            .map(|r| {
                let role_str: String = r.get("role");
                UserRole::from_str(&role_str).unwrap_or(UserRole::AccountingStaff)
            })
            .collect();

        Ok(User {
            id: user_row.get("id"),
            company_id: user_row.get("company_id"),
            email: user_row.get("email"),
            full_name: user_row.get("full_name"),
            password_hash: user_row.get("password_hash"),
            roles,
            is_active: user_row.get("is_active"),
            last_login_at: user_row.get::<Option<time::OffsetDateTime>, _>("last_login_at").map(|t| t.into()),
            created_at: user_row.get::<time::OffsetDateTime, _>("created_at").into(),
            updated_at: user_row.get::<time::OffsetDateTime, _>("updated_at").into(),
        })
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user_row = sqlx::query(
            r#"
            SELECT id, company_id, email, full_name, password_hash, is_active, last_login_at, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let user_row = match user_row {
            Some(row) => row,
            None => return Ok(None),
        };

        let role_rows = sqlx::query(
            r#"
            SELECT role
            FROM user_roles
            WHERE user_id = $1
            "#,
        )
        .bind(user_row.get::<Uuid, _>("id"))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let roles: Vec<UserRole> = role_rows
            .into_iter()
            .map(|r| {
                let role_str: String = r.get("role");
                UserRole::from_str(&role_str).unwrap_or(UserRole::AccountingStaff)
            })
            .collect();

        Ok(Some(User {
            id: user_row.get("id"),
            company_id: user_row.get("company_id"),
            email: user_row.get("email"),
            full_name: user_row.get("full_name"),
            password_hash: user_row.get("password_hash"),
            roles,
            is_active: user_row.get("is_active"),
            last_login_at: user_row.get::<Option<time::OffsetDateTime>, _>("last_login_at").map(|t| t.into()),
            created_at: user_row.get::<time::OffsetDateTime, _>("created_at").into(),
            updated_at: user_row.get::<time::OffsetDateTime, _>("updated_at").into(),
        }))
    }

    async fn save(&self, user: &User) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::Internal(e.into()))?;

        sqlx::query(
            r#"
            INSERT INTO users (id, company_id, email, full_name, password_hash, is_active, last_login_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(user.id)
        .bind(user.company_id)
        .bind(&user.email)
        .bind(&user.full_name)
        .bind(&user.password_hash)
        .bind(user.is_active)
        .bind(user.last_login_at)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        for role in &user.roles {
            let role_str = role.to_string();
            sqlx::query(
                r#"
                INSERT INTO user_roles (user_id, role)
                VALUES ($1, $2)
                ON CONFLICT (user_id, role) DO NOTHING
                "#,
            )
            .bind(user.id)
            .bind(role_str)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit().await.map_err(|e| AppError::Internal(e.into()))?;
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::Internal(e.into()))?;

        sqlx::query(
            r#"
            UPDATE users
            SET company_id = $2, email = $3, full_name = $4, password_hash = $5, is_active = $6, last_login_at = $7, updated_at = $8
            WHERE id = $1
            "#,
        )
        .bind(user.id)
        .bind(user.company_id)
        .bind(&user.email)
        .bind(&user.full_name)
        .bind(&user.password_hash)
        .bind(user.is_active)
        .bind(user.last_login_at)
        .bind(user.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        // Sync roles by deleting and re-inserting
        sqlx::query(
            r#"
            DELETE FROM user_roles
            WHERE user_id = $1
            "#,
        )
        .bind(user.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        for role in &user.roles {
            let role_str = role.to_string();
            sqlx::query(
                r#"
                INSERT INTO user_roles (user_id, role)
                VALUES ($1, $2)
                "#,
            )
            .bind(user.id)
            .bind(role_str)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit().await.map_err(|e| AppError::Internal(e.into()))?;
        Ok(())
    }
}
