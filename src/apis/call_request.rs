use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

///Calling the llm
///
///This function will return a result that will either be a string output of the llm or
///an dynamic error. This is good because rather than calling unwrap() on a bunch of things
///this function can now utilize the ? operator to automatically throw the error rather than
///having a chance to crash on a mishandled unwrap().
///
///Send allows for thread safety
pub async fn call_llm(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    //Extract API key
    let api_key: String =
        env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in enviornment variables");
    // let api_org: String =
    //     env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in enviornment variables");

    //Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    //Create headers
    let mut headers = HeaderMap::new();

    //Create api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    //Create org header
    // headers.insert(
    //     "OpenAI-Organization",
    //     HeaderValue::from_str(&format!("Bearer {}", api_org)).unwrap(),
    // );

    //Create Client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    //Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1,
    };

    //Troubleshooting
    // let res_raw = client
    //     .post(url)
    //     .json(&chat_completion)
    //     .send()
    //     .await
    //     .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;
    // dbg!(res_raw.text().await.unwrap());

    //sending post request to url,
    //waiting for response
    //once get response, call .json to provide json
    //gives result of APIResponse or Err
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    //Send response
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tets {
    use crate::models::general::llm::Message;

    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message: Message = Message {
            role: String::from("user"),
            content: String::from("This is a test. Provide a short response"),
        };
        let messages: Vec<Message> = vec![message];

        let res = call_llm(messages).await;

        match res {
            Ok(res_string) => {
                dbg!(res_string);
                assert!(true)
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}
