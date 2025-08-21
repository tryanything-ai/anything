use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "anything", table_name = "secrets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub secret_id: Uuid,
    pub account_id: Uuid,
    pub secret_name: String,
    pub secret_value_encrypted: Vec<u8>, // Encrypted bytea from pgsodium
    pub nonce: Vec<u8>, // Nonce used for encryption
    pub description: Option<String>,
    pub is_api_key: bool,
    pub archived: bool,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
