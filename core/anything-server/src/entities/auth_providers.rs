use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "auth_providers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub auth_provider_id: String,
    pub provider_name: String,
    pub provider_label: Option<String>,
    pub provider_icon: Option<String>,
    pub provider_description: Option<String>,
    pub provider_readme: Option<String>,
    pub auth_type: Option<String>,
    pub auth_url: Option<String>,
    pub token_url: Option<String>,
    pub access_token_lifetime_seconds: Option<i32>,
    pub refresh_token_lifetime_seconds: Option<i32>,
    pub scopes: Option<String>,
    pub public: Option<bool>,
    pub client_id_vault_id: Option<String>,
    pub client_secret_vault_id: Option<String>,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
