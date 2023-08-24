use std::sync::Arc;

use interface::startup;
use utils::{connect_db, init_emojis};

pub mod interface;
pub mod models;
pub mod usecase;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let pool = Arc::new(connect_db().await?);
    // init_emoji(&modules).await;
    startup(pool).await?;

    Ok(())
}
