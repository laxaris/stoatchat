use axum::{extract::{FromRef, FromRequestParts}, http::request::Parts};

use revolt_result::{create_error, Error, Result};

use crate::{Database, SsoUserInfo, User};

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for User
where
    Database: FromRef<S>,
    S: Send + Sync
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<User> {
        let db = Database::from_ref(state);

        // 1. Try bot token (existing)
        if let Some(Ok(bot_token)) = parts.headers.get("x-bot-token").map(|v| v.to_str()) {
            let bot = db.fetch_bot_by_token(bot_token).await?;
            return db.fetch_user(&bot.id).await;
        }

        // 2. Try session token (existing)
        if let Some(Ok(session_token)) =
            parts.headers.get("x-session-token").map(|v| v.to_str())
        {
            let session = db.fetch_session_by_token(session_token).await?;
            return db.fetch_user(&session.user_id).await;
        }

        // 3. Try SSO authentication (NEW)
        if let Some(sso_info) = SsoUserInfo::from_axum_headers(parts) {
            return db.fetch_or_create_sso_user(&sso_info).await;
        }

        Err(create_error!(NotAuthenticated))
    }
}
