use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Student table
        manager
            .create_table(
                Table::create()
                    .table(Student::Table)
                    .if_not_exists()
                    .col(pk_auto(Student::Id))
                    .col(string_uniq(Student::Name))
                    .col(string(Student::Avatars))
                    .col(string(Student::Prompt))
                    .to_owned(),
            )
            .await?;

        // Create Conversation table
        manager
            .create_table(
                Table::create()
                    .table(Conversation::Table)
                    .if_not_exists()
                    .col(string(Conversation::Id).primary_key())
                    .col(
                        timestamp_with_time_zone(Conversation::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Conversation::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(string(Conversation::Title).null())
                    .col(string_uniq(Conversation::StudentName))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Conversation::Table, Conversation::StudentName)
                            .to(Student::Table, Student::Name)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(Conversation::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Student::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Student {
    Table,
    Id,
    Name,
    Avatars,
    Prompt,
}

#[derive(DeriveIden)]
enum Conversation {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Title,
    StudentName,
}
