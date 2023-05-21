// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use once_cell::sync::OnceCell;
use sqlx::{PgPool, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub static POSTGRES: OnceCell<PgPool> = OnceCell::new();

#[inline]
pub fn dbconn() -> &'static PgPool {
    unsafe { POSTGRES.get_unchecked() }
}
