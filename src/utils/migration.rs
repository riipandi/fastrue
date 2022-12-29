use dialoguer::{theme::ColorfulTheme, Confirm};
use indicatif::{ProgressBar, ProgressIterator};
use sqlx::migrate::Migrator;
use std::path::PathBuf;

use crate::config::progressbar_style;

pub async fn run_migration() {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        match migrate_up().await {
            Ok(_) => println!("Database migration succes"),
            Err(e) => println!("Could not execute database migration: {:?}", e),
        }
    } else {
        println!("Sayonara 👋");
    }
}

async fn migrate_up() -> Result<(), sqlx::Error> {
    let migration_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let pool = crate::config::connection_pool().await;
    let migrator = Migrator::new(migration_dir).await.unwrap();

    let pb = ProgressBar::new(1000);
    pb.set_style(progressbar_style().unwrap());
    for _ in migrator.iter().progress_with(pb) {
        migrator.run(&pool).await.unwrap();
    }

    Ok(())
}
