// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use sea_query::{Iden, Order, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;

use crate::{entities, state};

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Uid,
    Aud,
    Role,
    Email,
    EmailConfirmedAt,
    EmailChangeTokenNew,
    EmailChange,
    EmailChangeSentAt,
    EmailChangeTokenCurrent,
    EmailChangeConfirmStatus,
    Phone,
    PhoneConfirmedAt,
    PhoneChange,
    PhoneChangeToken,
    PhoneChangeSentAt,
    InvitedAt,
    ConfirmationToken,
    ConfirmationSentAt,
    RecoveryToken,
    RecoverySentAt,
    ReauthenticationToken,
    ReauthenticationSentAt,
    LastSignInAt,
    RawAppMetaData,
    RawUserMetaData,
    IsSuperAdmin,
    IsSsoUser,
    ConfirmedAt,
    BannedUntil,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

pub async fn get_all() -> Result<Vec<entities::User>, sqlx::Error> {
    let (sql, values) = Query::select()
        .columns([
            Users::Id,
            Users::Uid,
            Users::Aud,
            Users::Role,
            Users::Email,
            Users::EmailConfirmedAt,
            Users::EmailChangeTokenNew,
            Users::EmailChange,
            Users::EmailChangeSentAt,
            Users::EmailChangeTokenCurrent,
            Users::EmailChangeConfirmStatus,
            Users::Phone,
            Users::PhoneConfirmedAt,
            Users::PhoneChange,
            Users::PhoneChangeToken,
            Users::PhoneChangeSentAt,
            Users::InvitedAt,
            Users::ConfirmationToken,
            Users::ConfirmationSentAt,
            Users::RecoveryToken,
            Users::RecoverySentAt,
            Users::ReauthenticationToken,
            Users::ReauthenticationSentAt,
            Users::LastSignInAt,
            Users::RawAppMetaData,
            Users::RawUserMetaData,
            Users::IsSuperAdmin,
            Users::IsSsoUser,
            Users::ConfirmedAt,
            Users::BannedUntil,
            Users::CreatedAt,
            Users::UpdatedAt,
            Users::DeletedAt,
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
