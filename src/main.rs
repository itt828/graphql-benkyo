use std::sync::Arc;

use interface::{modules::Modules, startup};

pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod usecase;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let modules = Arc::new(Modules::new().await?);
    startup(modules).await?;

    Ok(())
}