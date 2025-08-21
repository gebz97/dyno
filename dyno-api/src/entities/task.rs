use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub workflow_id: Option<u32>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::inventory::Entity")]
    Inventory,
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}