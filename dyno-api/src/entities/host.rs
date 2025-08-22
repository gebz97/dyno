use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "hosts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub namespace_id: u32,   // FK
    pub inventory_id: u32,   // FK
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::namespace::Entity", from = "Column::NamespaceId", to = "super::namespace::Column::Id")]
    Namespace,
    #[sea_orm(belongs_to = "super::inventory::Entity", from = "Column::InventoryId", to = "super::inventory::Column::Id")]
    Inventory,
}

// host.rs
impl Related<super::namespace::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Namespace.def()
  }
}

impl Related<super::inventory::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Inventory.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
