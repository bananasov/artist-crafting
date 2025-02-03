use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tags::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tags::ArchiveName).text().not_null())
                    .col(ColumnDef::new(Tags::PathInArchive).text().not_null())
                    .col(ColumnDef::new(Tags::Namespace).text().null())
                    .col(ColumnDef::new(Tags::Replace).boolean().null())
                    .col(ColumnDef::new(Tags::TagValues).text().not_null())
                    .col(
                        ColumnDef::new(Tags::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Recipes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Recipes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Recipes::ArchiveName).text().not_null())
                    .col(ColumnDef::new(Recipes::PathInArchive).text().not_null())
                    .col(ColumnDef::new(Recipes::Namespace).text().null())
                    .col(ColumnDef::new(Recipes::RecipeType).text().not_null())
                    .col(ColumnDef::new(Recipes::RecipeData).json().not_null())
                    .col(
                        ColumnDef::new(Recipes::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_tags_archive_name")
                    .table(Tags::Table)
                    .col(Tags::ArchiveName)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tags_namespace")
                    .table(Tags::Table)
                    .col(Tags::Namespace)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tags_archive_path")
                    .table(Tags::Table)
                    .col(Tags::ArchiveName)
                    .col(Tags::PathInArchive)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_recipes_archive_name")
                    .table(Recipes::Table)
                    .col(Recipes::ArchiveName)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_recipes_namespace")
                    .table(Recipes::Table)
                    .col(Recipes::Namespace)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_recipes_recipe_type")
                    .table(Recipes::Table)
                    .col(Recipes::RecipeType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_recipes_archive_path")
                    .table(Recipes::Table)
                    .col(Recipes::ArchiveName)
                    .col(Recipes::PathInArchive)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Recipes::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tags {
    Table,
    Id,
    ArchiveName,
    PathInArchive,
    Namespace,
    Replace,
    TagValues,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Recipes {
    Table,
    Id,
    ArchiveName,
    PathInArchive,
    Namespace,
    RecipeType,
    RecipeData,
    CreatedAt,
}
