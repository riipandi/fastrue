// Copyright 2022-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0

use dialoguer::{theme::ColorfulTheme, Confirm};
use indicatif::{ProgressBar, ProgressIterator};
use sqlx::migrate::Migrator;
use std::path::PathBuf;

use crate::{config::progressbar_style, state};

pub async fn run_migration(force: bool) {
    let success_message = "🍀 Database migration succes";
    let failed_message = "🍀 Could not execute database migration";

    if force {
        let success_message = success_message;
        match migrate_up().await {
            Ok(_) => tracing::info!("{}", success_message),
            Err(e) => tracing::info!("{}: {:?}", failed_message, e),
        }
    } else if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        let success_message = success_message;
        match migrate_up().await {
            Ok(_) => tracing::info!("{}", success_message),
            Err(e) => tracing::debug!("{}: {:?}", failed_message, e),
        }
    } else {
        println!("Sayonara 👋");
    }
}

async fn migrate_up() -> Result<(), sqlx::Error> {
    let migration_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let migrator = Migrator::new(migration_dir).await.unwrap();

    let pb = ProgressBar::new(1000);
    pb.set_style(progressbar_style().unwrap());
    for _ in migrator.iter().progress_with(pb) {
        migrator.run(state::dbconn()).await.unwrap();
    }

    Ok(())
}
