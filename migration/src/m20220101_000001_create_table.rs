use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, mgr: &SchemaManager) -> Result<(), DbErr> {
        // namespaces
        mgr.create_table(
            Table::create()
                .table(Namespaces::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Namespaces::Id)
                        .integer()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Namespaces::Name).string().not_null())
                .to_owned(),
        )
        .await?;

        // inventories
        mgr.create_table(
            Table::create()
                .table(Inventories::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Inventories::Id)
                        .integer()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Inventories::Name).string().not_null())
                .col(
                    ColumnDef::new(Inventories::NamespaceId)
                        .integer()
                        .not_null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(Inventories::Table, Inventories::NamespaceId)
                        .to(Namespaces::Table, Namespaces::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // hosts
        mgr.create_table(
            Table::create()
                .table(Hosts::Table)
                .if_not_exists()
                .col(ColumnDef::new(Hosts::Id).integer().not_null().primary_key())
                .col(ColumnDef::new(Hosts::Name).string().not_null())
                .col(ColumnDef::new(Hosts::NamespaceId).integer().not_null())
                .col(ColumnDef::new(Hosts::InventoryId).integer().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .from(Hosts::Table, Hosts::NamespaceId)
                        .to(Namespaces::Table, Namespaces::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(Hosts::Table, Hosts::InventoryId)
                        .to(Inventories::Table, Inventories::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // workflows
        mgr.create_table(
            Table::create()
                .table(Workflows::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Workflows::Id)
                        .integer()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Workflows::Name).string().not_null())
                .col(ColumnDef::new(Workflows::NamespaceId).integer().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .from(Workflows::Table, Workflows::NamespaceId)
                        .to(Namespaces::Table, Namespaces::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // tasks
        mgr.create_table(
            Table::create()
                .table(Tasks::Table)
                .if_not_exists()
                .col(ColumnDef::new(Tasks::Id).integer().not_null().primary_key())
                .col(ColumnDef::new(Tasks::Name).string().not_null())
                .col(ColumnDef::new(Tasks::WorkflowId).integer().null())
                .col(ColumnDef::new(Tasks::InventoryId).integer().null())
                .foreign_key(
                    ForeignKey::create()
                        .from(Tasks::Table, Tasks::WorkflowId)
                        .to(Workflows::Table, Workflows::Id)
                        .on_delete(ForeignKeyAction::SetNull),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(Tasks::Table, Tasks::InventoryId)
                        .to(Inventories::Table, Inventories::Id)
                        .on_delete(ForeignKeyAction::SetNull),
                )
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, mgr: &SchemaManager) -> Result<(), DbErr> {
        mgr.drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await?;
        mgr.drop_table(Table::drop().table(Workflows::Table).to_owned())
            .await?;
        mgr.drop_table(Table::drop().table(Hosts::Table).to_owned())
            .await?;
        mgr.drop_table(Table::drop().table(Inventories::Table).to_owned())
            .await?;
        mgr.drop_table(Table::drop().table(Namespaces::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Namespaces {
    Table,
    Id,
    Name,
}
#[derive(Iden)]
enum Inventories {
    Table,
    Id,
    Name,
    NamespaceId,
}
#[derive(Iden)]
enum Hosts {
    Table,
    Id,
    Name,
    NamespaceId,
    InventoryId,
}
#[derive(Iden)]
enum Workflows {
    Table,
    Id,
    Name,
    NamespaceId,
}
#[derive(Iden)]
enum Tasks {
    Table,
    Id,
    Name,
    WorkflowId,
    InventoryId,
}
