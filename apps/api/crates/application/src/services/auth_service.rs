use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    dto::auth::{
        LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse, UserSummary,
        CreateUserRequest, UpdateUserRequest,
    },
    errors::AppError,
    ports::user_repository::UserRepository,
};
use finance_assistant_domain::entities::user::User;

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    jwt_secret: String,
    jwt_access_minutes: i64,
    jwt_refresh_days: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub company_id: String,
    pub roles: Vec<String>,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String, // "access" or "refresh"
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        jwt_secret: String,
        jwt_access_minutes: i64,
        jwt_refresh_days: i64,
    ) -> Self {
        Self {
            user_repo,
            jwt_secret,
            jwt_access_minutes,
            jwt_refresh_days,
        }
    }

    /// Hash a raw password string using Argon2id.
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Password hashing failed: {}", e)))
    }

    /// Authenticate a user by email and password, updating the last login timestamp and returning JWT tokens.
    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, AppError> {
        let user = self.user_repo.find_by_email(&req.email).await?;
        let mut user = match user {
            Some(u) => u,
            None => {
                return Err(AppError::Unauthorized {
                    reason: "Invalid email or password".to_string(),
                })
            }
        };

        if !user.is_active {
            return Err(AppError::Unauthorized {
                reason: "User account is inactive".to_string(),
            });
        }

        // Verify password
        let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Invalid password hash format: {}", e))
        })?;

        Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Unauthorized {
                reason: "Invalid email or password".to_string(),
            })?;

        // Update last login
        user.last_login_at = Some(time::OffsetDateTime::now_utc());
        self.user_repo.update(&user).await?;

        // Generate tokens
        let access_token = self.generate_token(&user, "access", self.jwt_access_minutes)?;
        let refresh_token =
            self.generate_token(&user, "refresh", self.jwt_refresh_days * 24 * 60)?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            user: UserSummary {
                id: user.id,
                company_id: user.company_id,
                email: user.email,
                full_name: user.full_name,
                roles: user.roles.iter().map(|r| r.to_string()).collect(),
            },
        })
    }

    /// Refresh access and refresh tokens using a valid refresh token.
    pub async fn refresh(
        &self,
        req: RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, AppError> {
        // Decode token
        let token_data = jsonwebtoken::decode::<Claims>(
            &req.refresh_token,
            &jsonwebtoken::DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|e| AppError::Unauthorized {
            reason: format!("Invalid or expired refresh token: {}", e),
        })?;

        let claims = token_data.claims;
        if claims.token_type != "refresh" {
            return Err(AppError::Unauthorized {
                reason: "Invalid token type".to_string(),
            });
        }

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized {
            reason: "Invalid user ID format in token".to_string(),
        })?;
        let user = self.user_repo.find_by_id(user_id).await?;

        if !user.is_active {
            return Err(AppError::Unauthorized {
                reason: "User account is inactive".to_string(),
            });
        }

        // Generate new tokens
        let access_token = self.generate_token(&user, "access", self.jwt_access_minutes)?;
        let refresh_token =
            self.generate_token(&user, "refresh", self.jwt_refresh_days * 24 * 60)?;

        Ok(RefreshTokenResponse {
            access_token,
            refresh_token,
        })
    }

    fn generate_token(
        &self,
        user: &User,
        token_type: &str,
        duration_minutes: i64,
    ) -> Result<String, AppError> {
        let iat = time::OffsetDateTime::now_utc().unix_timestamp();
        let exp = iat + (duration_minutes * 60);

        let claims = Claims {
            sub: user.id.to_string(),
            company_id: user.company_id.to_string(),
            roles: user.roles.iter().map(|r| r.to_string()).collect(),
            exp,
            iat,
            token_type: token_type.to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(e.into()))
    }

    pub async fn list_users(&self, company_id: Uuid) -> Result<Vec<UserSummary>, AppError> {
        let users = self.user_repo.find_by_company(company_id).await?;
        let summaries = users
            .into_iter()
            .map(|u| UserSummary {
                id: u.id,
                company_id: u.company_id,
                email: u.email,
                full_name: u.full_name,
                roles: u.roles.iter().map(|r| r.to_string()).collect(),
            })
            .collect();
        Ok(summaries)
    }

    pub async fn create_user(&self, creator_company_id: Uuid, req: CreateUserRequest) -> Result<UserSummary, AppError> {
        if let Some(_) = self.user_repo.find_by_email(&req.email).await? {
            return Err(AppError::Validation {
                message: format!("User with email '{}' already exists", req.email),
            });
        }

        let password_hash = Self::hash_password(&req.password)?;

        let mut roles = Vec::new();
        for r in req.roles {
            if let Ok(role) = std::str::FromStr::from_str(&r) {
                roles.push(role);
            }
        }

        let new_user = User {
            id: Uuid::new_v4(),
            company_id: creator_company_id,
            email: req.email,
            full_name: req.full_name,
            password_hash,
            roles,
            is_active: true,
            last_login_at: None,
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
        };

        self.user_repo.save(&new_user).await?;

        Ok(UserSummary {
            id: new_user.id,
            company_id: new_user.company_id,
            email: new_user.email,
            full_name: new_user.full_name,
            roles: new_user.roles.iter().map(|r| r.to_string()).collect(),
        })
    }

    pub async fn update_user(
        &self,
        company_id: Uuid,
        user_id: Uuid,
        req: UpdateUserRequest,
    ) -> Result<UserSummary, AppError> {
        let mut user = self.user_repo.find_by_id(user_id).await?;

        if user.company_id != company_id {
            return Err(AppError::Unauthorized {
                reason: "Cannot manage users in other companies".to_string(),
            });
        }

        let mut roles = Vec::new();
        for r in req.roles {
            if let Ok(role) = std::str::FromStr::from_str(&r) {
                roles.push(role);
            }
        }

        user.full_name = req.full_name;
        user.roles = roles;
        user.is_active = req.is_active;
        user.updated_at = time::OffsetDateTime::now_utc();

        self.user_repo.update(&user).await?;

        Ok(UserSummary {
            id: user.id,
            company_id: user.company_id,
            email: user.email,
            full_name: user.full_name,
            roles: user.roles.iter().map(|r| r.to_string()).collect(),
        })
    }

    pub async fn delete_user(&self, company_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let user = self.user_repo.find_by_id(user_id).await?;

        if user.company_id != company_id {
            return Err(AppError::Unauthorized {
                reason: "Cannot manage users in other companies".to_string(),
            });
        }

        self.user_repo.delete(user_id).await?;
        Ok(())
    }
}


