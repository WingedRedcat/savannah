use savannah::{Engine, EngineConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://example.com".to_string());
    let engine = Engine::new(EngineConfig::default())?;
    let page = engine.navigate(&url).await?;
    println!(
        "loaded {} (HTTP {}, {} bytes)",
        page.url,
        page.status,
        page.body.len()
    );
    engine.shutdown().await;
    Ok(())
}
