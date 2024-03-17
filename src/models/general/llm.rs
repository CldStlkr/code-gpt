///framework that the llm takes input from
///essentially a 1-1 conversion of the python code provided
///in the OpenAI information section on their API page
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]

pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32, //temperature is how random/creative the model is allowed to be
}

///These structs compartmentalize the structure
///of the message provided by the llm

#[derive(Debug, Deserialize)]
pub struct APIMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct APIChoice {
    pub message: APIMessage,
}

#[derive(Debug, Deserialize)]
pub struct APIResponse {
    pub choices: Vec<APIChoice>,
}
