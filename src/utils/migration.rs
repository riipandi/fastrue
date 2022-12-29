use dialoguer::{theme::ColorfulTheme, Confirm};
use sqlx::migrate::Migrator;
use std::path::PathBuf;

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
    migrator.run(&pool).await.unwrap();

    Ok(())
}
