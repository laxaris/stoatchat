use revolt_database::{Database, User};
use revolt_result::Result;

use rocket::State;

/// Acknowledge Policy Changes
///
/// Accept/acknowledge changes to platform policy.
#[utoipa::path(
    tag = "Policy",
    security(("Session-Token" = []), ("Bot-Token" = [])),
    responses(
        (status = 204),
    ),
)]
#[post("/acknowledge")]
pub async fn acknowledge_policy_changes(db: &State<Database>, user: User) -> Result<()> {
    db.acknowledge_policy_changes(&user.id).await
}
