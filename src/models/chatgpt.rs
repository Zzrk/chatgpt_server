use async_openai::{
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use rocket::serde::json::serde_json;
use std::error::Error;

pub async fn chat(question: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            // ChatCompletionRequestSystemMessageArgs::default()
            //     .content("You are a helpful assistant.")
            //     .build()?
            //     .into(),
            // ChatCompletionRequestUserMessageArgs::default()
            //     .content("Who won the world series in 2020?")
            //     .build()?
            //     .into(),
            // ChatCompletionRequestAssistantMessageArgs::default()
            //     .content("The Los Angeles Dodgers won the World Series in 2020.")
            //     .build()?
            //     .into(),
            // ChatCompletionRequestUserMessageArgs::default()
            //     .content("Where was it played?")
            //     .build()?
            //     .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(question)
                .build()?
                .into(),
        ])
        .build()?;

    println!("\n{}\n", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");

    let answer: Option<&Option<String>> = response
        .choices
        .first()
        .map(|choice| &choice.message.content);

    let res = match answer {
        Some(inner) => match inner {
            Some(s) => s.to_string(),
            None => String::from("No answer"),
        },
        None => String::from("No answer"),
    };

    print!("\n{}\n", res);

    Ok(res)
}
