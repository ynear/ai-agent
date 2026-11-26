use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_stream::stream;
use futures::{Stream, StreamExt};

pub fn chat_stream(
    model: &str,
    system: Option<&str>,
    prompt: &str,
) -> impl Stream<Item = anyhow::Result<String>> {
    stream! {
        let client = async_openai::Client::new();
        let mut msgs = vec![];

        if let Some(system) = system {
            msgs.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system)
                    .build()?
                    .into(),
            )
        }

        msgs.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        );

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(msgs)
            .max_tokens(2048u32)
            .build()?;

        let mut stream = client.chat().create_stream(request).await?;

        while let Some(res_result) = stream.next().await {
            match res_result {
                Ok(chunk) => {
                    if let Some(choice) = chunk.choices.first() && let Some(new_text) = &choice.delta.content {
                            yield Ok(new_text.clone())

                    }
                }
                Err(err) => yield Err(err.into()),
            }
        }
    }
}
