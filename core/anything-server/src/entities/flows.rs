use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "anything", table_name = "flows")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub flow_id: Uuid,
    pub account_id: Uuid,
    pub flow_name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub long_description: Option<String>,
    pub header_image: Option<String>,
    pub marketplace_flow_template_id: Option<Uuid>,
    pub active: bool,
    pub archived: bool,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::flow_versions::Entity")]
    FlowVersions,
    #[sea_orm(has_many = "super::tasks::Entity")]
    Tasks,
}

impl Related<super::flow_versions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowVersions.def()
    }
}

impl Related<super::tasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tasks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
