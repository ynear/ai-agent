use crate::models::action_plan::ActionPlan;
use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
};

pub async fn chat_complete_structured(
    model: &str,
    system: Option<&str>,
    prompt: &str,
) -> anyhow::Result<ActionPlan> {
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

    let schema = schemars::schema_for!(ActionPlan);
    let schema_json = schema.as_value().clone();
    let format_setting = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description: Some(
                "A step-by-step agent action plan with difficulty and time estimate".into(),
            ),
            name: "action_plan".into(),
            schema: schema_json,
            strict: Some(true),
        },
    };

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(msgs)
        .response_format(format_setting)
        .max_tokens(2048u32)
        .build()?;

    let res = client.chat().create(request).await?;

    let plan: ActionPlan = res
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .ok_or_else(|| anyhow::anyhow!("响应为空!"))
        .and_then(|s| serde_json::from_str(&s).map_err(Into::into))?;

    Ok(plan)
}
