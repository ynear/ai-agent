use ai_agent::constant;
use ai_agent::llm::structured;
use anyhow::{Ok, Result};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let content = structured::chat_complete_structured(
        constant::NVIDIA_NEMOTRON_3_MODEL,
        Some("你是一个全能助手"),
        "爱尔兰的首都是哪里",
    )
    .await?;

    println!("{:#?}", content);
    Ok(())
}
