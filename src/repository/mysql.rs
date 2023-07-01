use sqlx::mysql::MySqlConnectOptions;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

pub mod blog;

pub async fn connect_db() -> anyhow::Result<MySqlPool> {
    let mysql_config = MySqlConnectOptions::new()
        .host("")
        .username("")
        .password("")
        .database("")
        .port(1111);
    let pool = MySqlPoolOptions::new().connect_with(mysql_config).await?;
    Ok(pool)
}
