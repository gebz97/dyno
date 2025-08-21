use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "namespaces")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::inventory::Entity")]
    Inventory,
    #[sea_orm(has_many = "super::host::Entity")]
    Host,
    #[sea_orm(has_many = "super::workflow::Entity")]
    Workflow,
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl Related<super::host::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl Related<super::workflow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workflow.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}