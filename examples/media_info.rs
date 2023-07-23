use consumet_api_rs::models::MovieParser;
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ.fetch_media_info("tv/watch-yo-mtv-raps-82018".to_owned()).await?
    );
    
    Ok(())
}