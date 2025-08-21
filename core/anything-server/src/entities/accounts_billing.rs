use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "accounts_billing")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub account_id: Uuid,
    pub billing_status: Option<String>,
    pub plan: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub active: Option<bool>,
    pub created_at: Option<ChronoDateTimeUtc>,
    pub updated_at: Option<ChronoDateTimeUtc>,
    pub tasks_used: Option<i32>,
    pub tasks_limit: Option<i32>,
    pub storage_used: Option<i64>,
    pub storage_limit: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
