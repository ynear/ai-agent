use ai_agent::constant;
use ai_agent::llm::stream;
use anyhow::Result;
use futures::StreamExt;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let s = stream::chat_stream(
        constant::NVIDIA_NEMOTRON_3_MODEL,
        Some("你是一个全能助手"),
        "道德经的第四章是啥",
    );

    futures::pin_mut!(s);
    let mut output = String::new();
    while let Some(result) = s.next().await {
        match result {
            Ok(txt) => {
                output.push_str(&txt);
                print!("{txt}");
            }
            Err(err) => {
                tracing::error!("error");
                return Err(err);
            }
        }
    }

    println!("^^^^^^^^^^^^^^^^^^^^^^");
    println!("{output}");

    Ok(())
}
