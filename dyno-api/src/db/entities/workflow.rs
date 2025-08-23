use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workflows")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub namespace_id: u32,   // FK
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {    
    #[sea_orm(belongs_to = "super::namespace::Entity", from = "Column::NamespaceId", to = "super::namespace::Column::Id")]
    Namespace,
    #[sea_orm(has_many = "super::task::Entity")]
    Task,
}

// workflow.rs
impl Related<super::namespace::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Namespace.def()
  }
}

impl Related<super::task::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Task.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
