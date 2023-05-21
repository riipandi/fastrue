// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use sea_query::{Iden, Order, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;

use crate::{entities, state};

#[derive(Iden)]
enum Users {
    Table,
    InstanceId,
    Id,
    Aud,
    Role,
    Email,
    EncryptedPassword,
    ConfirmedAt,
    InvitedAt,
    ConfirmationToken,
    ConfirmationSentAt,
    RecoveryToken,
    RecoverySentAt,
    EmailChangeToken,
    EmailChange,
    EmailChangeSentAt,
    LastSignInAt,
    RawAppMetaData,
    RawUserMetaData,
    IsSuperAdmin,
    CreatedAt,
    UpdatedAt,
}

pub async fn get_all() -> Result<Vec<entities::User>, sqlx::Error> {
    let (sql, values) = Query::select()
        .columns([
            Users::InstanceId,
            Users::Id,
            Users::Aud,
            Users::Role,
            Users::Email,
            Users::EncryptedPassword,
            Users::ConfirmedAt,
            Users::InvitedAt,
            Users::ConfirmationToken,
            Users::ConfirmationSentAt,
            Users::RecoveryToken,
            Users::RecoverySentAt,
            Users::EmailChangeToken,
            Users::EmailChange,
            Users::EmailChangeSentAt,
            Users::LastSignInAt,
            Users::RawAppMetaData,
            Users::RawUserMetaData,
            Users::IsSuperAdmin,
            Users::CreatedAt,
            Users::UpdatedAt,
        ])
        .from(Users::Table)
        .order_by(Users::Id, Order::Desc)
        .limit(1)
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, entities::User, _>(&sql, values.clone())
        .fetch_all(state::dbconn())
        .await
        .unwrap();

    Ok(rows)
}
