use anyhow::Result;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost:6379")?;

    let mut conn = client.get_async_connection().await?;

    redis::cmd("SET")
        .arg("Taro")
        .arg("I am Taro.")
        .query_async(&mut conn)
        .await?;

    redis::cmd("SET")
        .arg("Jiro")
        .arg("I am Jiro.")
        .query_async(&mut conn)
        .await?;

    let result: (String, String) = redis::cmd("MGET")
        .arg(&["Taro", "Jiro"])
        .query_async(&mut conn)
        .await?;

    println!("{:?}", result);

    Ok(())
}
