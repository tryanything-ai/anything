use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "anything", table_name = "flow_versions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub flow_version_id: Uuid,
    pub account_id: Uuid,
    pub flow_id: Uuid,
    pub archived: bool,
    pub description: Option<String>,
    pub from_template: bool,
    pub parent_flow_template_id: Option<Uuid>,
    pub parent_flow_version_id: Option<Uuid>,
    pub published: bool,
    pub published_at: Option<DateTimeWithTimeZone>,
    pub un_published: bool,
    pub un_published_at: Option<DateTimeWithTimeZone>,
    pub flow_definition: Json,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::flows::Entity",
        from = "Column::FlowId",
        to = "super::flows::Column::FlowId"
    )]
    Flows,
    #[sea_orm(has_many = "super::tasks::Entity")]
    Tasks,
}

impl Related<super::flows::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Flows.def()
    }
}

impl Related<super::tasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tasks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
