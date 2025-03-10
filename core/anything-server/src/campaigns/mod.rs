use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;
use uuid::Uuid;

mod contacts;
mod create;
mod delete;
mod get;
// mod status;
// mod engine;
mod update;

pub use contacts::*;
pub use create::*;
pub use delete::*;
pub use get::*;
// pub use status::*;
// pub use engine::*;
pub use update::*;

pub use contacts::{
    add_contacts_to_campaign, create_and_add_contacts_to_campaign, get_campaign_contacts,
    remove_contact_from_campaign,
};
pub use create::create_campaign;
pub use delete::delete_campaign;
// pub use engine::campaign_engine_loop;
pub use get::{get_campaign, get_campaigns};
pub use update::update_campaign;
