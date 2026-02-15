use revolt_result::Result;
use serde::{Deserialize, Serialize};

use crate::{Database, User};
use crate::models::users::ops::AbstractUsers;

/// SSO user information from OAuth2 Proxy headers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoUserInfo {
    /// User's email from SSO provider
    pub email: String,
    /// Username from SSO provider (optional)
    pub username: Option<String>,
    /// Access token from SSO provider (optional, for additional API calls)
    pub access_token: Option<String>,
}

impl SsoUserInfo {
    /// Extract SSO user info from Axum request headers
    #[cfg(feature = "axum-impl")]
    pub fn from_axum_headers(parts: &axum::http::request::Parts) -> Option<Self> {
        let email = parts
            .headers
            .get("x-auth-email")
            .or_else(|| parts.headers.get("x-auth-request-email"))
            .or_else(|| parts.headers.get("x-forwarded-email"))
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())?;

        let username = parts
            .headers
            .get("x-forwarded-preferred-username")
            .or_else(|| parts.headers.get("x-auth-user"))
            .or_else(|| parts.headers.get("x-auth-request-user"))
            .or_else(|| parts.headers.get("x-forwarded-user"))
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let access_token = parts
            .headers
            .get("x-auth-access-token")
            .or_else(|| parts.headers.get("x-auth-request-access-token"))
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Some(SsoUserInfo {
            email,
            username,
            access_token,
        })
    }

    /// Extract SSO user info from Rocket request headers
    #[cfg(feature = "rocket-impl")]
    pub fn from_rocket_headers(headers: &rocket::http::HeaderMap<'_>) -> Option<Self> {
        let email = headers
            .get("x-auth-email")
            .next()
            .or_else(|| headers.get("x-auth-request-email").next())
            .or_else(|| headers.get("x-forwarded-email").next())
            .map(|s| s.to_string())?;

        let username = headers
            .get("x-forwarded-preferred-username")
            .next()
            .or_else(|| headers.get("x-auth-user").next())
            .or_else(|| headers.get("x-auth-request-user").next())
            .or_else(|| headers.get("x-forwarded-user").next())
            .map(|s| s.to_string());

        let access_token = headers
            .get("x-auth-access-token")
            .next()
            .or_else(|| headers.get("x-auth-request-access-token").next())
            .map(|s| s.to_string());

        Some(SsoUserInfo {
            email,
            username,
            access_token,
        })
    }
}

impl Database {
    /// Fetch or create user from SSO information
    pub async fn fetch_or_create_sso_user(&self, sso_info: &SsoUserInfo) -> Result<User> {
        // Try to find existing user by email
        if let Ok(user) = self.fetch_user_by_email(&sso_info.email).await {
            return Ok(user);
        }

        // Create new user from SSO info
        self.create_sso_user(sso_info).await
    }

    /// Fetch user by email
    pub async fn fetch_user_by_email(&self, email: &str) -> Result<User> {
        // This is implemented by the database driver
        match self {
            Database::Reference(db) => db.fetch_user_by_email(email).await,
            #[cfg(feature = "mongodb")]
            Database::MongoDb(db) => db.fetch_user_by_email(email).await,
        }
    }

    /// Create a new user from SSO information
    pub async fn create_sso_user(&self, sso_info: &SsoUserInfo) -> Result<User> {
        use ulid::Ulid;

        // Generate username from email if not provided
        let username = sso_info.username.clone().unwrap_or_else(|| {
            sso_info
                .email
                .split('@')
                .next()
                .unwrap_or("user")
                .to_string()
        });

        // Create user object
        let user = User {
            id: Ulid::new().to_string(),
            username: username.clone(),
            discriminator: format!("{:04}", rand::random::<u16>() % 10000),
            display_name: Some(username),
            avatar: None,
            relations: None,
            badges: None,
            status: None,
            profile: None,
            flags: None,
            privileged: false,
            bot: None,
            suspended_until: None,
            last_acknowledged_policy_change: iso8601_timestamp::Timestamp::UNIX_EPOCH,
        };

        // Insert user into database
        self.insert_user(&user).await?;

        log::info!("Created new SSO user: {} ({})", user.username, sso_info.email);

        Ok(user)
    }
}
