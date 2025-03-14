use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建消息表
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(pk_auto(Message::Id))
                    .col(string(Message::ConversationId))
                    .col(string(Message::Role))
                    .col(text(Message::Content))
                    .col(string(Message::Name).null())
                    .col(
                        timestamp_with_time_zone(Message::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(integer(Message::Index))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Message::Table, Message::ConversationId)
                            .to(Conversation::Table, Conversation::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 添加索引以加快查询速度
        manager
            .create_index(
                Index::create()
                    .table(Message::Table)
                    .name("idx_message_conversation_id")
                    .col(Message::ConversationId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Message::Table)
                    .name("idx_message_index")
                    .col(Message::Index)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除索引
        manager
            .drop_index(
                Index::drop()
                    .table(Message::Table)
                    .name("idx_message_conversation_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .table(Message::Table)
                    .name("idx_message_index")
                    .to_owned(),
            )
            .await?;

        // 删除消息表
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await?;

        Ok(())
    }
}

// 已有的表定义（从m20220101_000001_create_table.rs中引用）
#[derive(DeriveIden)]
enum Conversation {
    Table,
    Id,
}

// 新增的消息表定义
#[derive(DeriveIden)]
enum Message {
    Table,
    Id,
    ConversationId,
    Role,      // 消息角色：system, user, assistant, tool
    Content,   // 消息内容
    Name,      // 可选的名称字段
    CreatedAt, // 创建时间
    Index,     // 消息在对话中的顺序索引
}
