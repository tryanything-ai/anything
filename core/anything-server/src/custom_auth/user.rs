use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User type that replaces the Supabase JWT User
/// This provides compatibility for existing code while using our custom auth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub account_id: String, // Current account context
    pub jwt: String, // For backward compatibility with existing code
}

impl User {
    /// Create a User with JWT - required for backward compatibility
    pub fn from_auth_user_with_jwt(user: &crate::entities::users::Model, account_id: String, jwt: String) -> Self {
        Self {
            id: user.user_id,
            email: user.email.clone(),
            username: user.username.clone(),
            account_id,
            jwt,
        }
    }
}

/// Legacy User type alias for backward compatibility
pub type LegacyUser = User;
