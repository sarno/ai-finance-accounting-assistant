use std::sync::Arc;
use uuid::Uuid;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};

use finance_assistant_domain::entities::user::User;
use crate::{
    dto::auth::{LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse, UserSummary},
    errors::AppError,
    ports::user_repository::UserRepository,
};

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
            None => return Err(AppError::Unauthorized {
                reason: "Invalid email or password".to_string(),
            }),
        };

        if !user.is_active {
            return Err(AppError::Unauthorized {
                reason: "User account is inactive".to_string(),
            });
        }

        // Verify password
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid password hash format: {}", e)))?;
        
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
        let refresh_token = self.generate_token(&user, "refresh", self.jwt_refresh_days * 24 * 60)?;

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
    pub async fn refresh(&self, req: RefreshTokenRequest) -> Result<RefreshTokenResponse, AppError> {
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
        let refresh_token = self.generate_token(&user, "refresh", self.jwt_refresh_days * 24 * 60)?;

        Ok(RefreshTokenResponse {
            access_token,
            refresh_token,
        })
    }

    fn generate_token(&self, user: &User, token_type: &str, duration_minutes: i64) -> Result<String, AppError> {
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
}
