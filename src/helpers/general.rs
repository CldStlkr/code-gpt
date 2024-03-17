use std::fs;

use regex::Regex;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

use crate::{
    apis::call_request::call_llm, helpers::command_line::PrintCommand,
    models::general::llm::Message,
};

pub const CODE_TEMPLATE_PATH: &str = "placeholder";
pub const WEB_SERVER_PROJECT_PATH: &str = "placeholder";
pub const EXEC_MAIN_PATH: &str = "placeholder";
pub const API_SCHEMA_PATH: &str = "placeholder";

//Extend ai function to encourage specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_func_str = ai_func(func_input);

    //Extend string to encourge llm to only print output
    let msg: String = format!(
        "FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the results of functions.
    Nothing else. No commentary. Here is the input to the function: {}",
        ai_func_str, func_input
    );

    Message {
        role: String::from("system"),
        content: msg,
    }
}

// Custom deserialization function to convert a string to a boolean
pub fn deserialize_string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.as_ref() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(serde::de::Error::custom(format!(
            "expected true or false, found {}",
            s
        ))),
    }
}

//Performs call to LLM
pub async fn ai_task_request(
    msg_context: String,
    agent_role: &str,
    agent_operation: &str,
    passed_function: for<'a> fn(&'a str) -> &'static str,
) -> String {
    let extended_msg: Message = extend_ai_function(passed_function, &msg_context);

    //Print current status
    PrintCommand::AICall.print_agent_msg(agent_role, agent_operation);

    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_llm(vec![extended_msg.clone()]).await;

    //Return successfully or try again (only once though)
    match llm_response_res {
        Ok(llm_response) => llm_response,
        Err(_) => call_llm(vec![extended_msg.clone()])
            .await
            .expect("Failed to call OpenAI twice"),
    }
}

//Decoded Version
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_role: &str,
    agent_operation: &str,
    passed_function: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_role, agent_operation, passed_function).await;
    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai response from serde_json");

    decoded_response
}

//Check if request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

//Get Code Template
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

//Get Exec Main
pub fn read_exec_main_contents() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

//Save New Backend Code
pub fn save_backend_code(contents: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write main.rs file");
}

//Save API Endpoints
pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API endpoints to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_func() {
        let extended_msg: Message =
            extend_ai_function(convert_user_input_to_goal, "something something");
        assert_eq!(extended_msg.role, String::from("system"));
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param: String = String::from("Make me a webserver for a streaming website");
        let result = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(result.len() > 20);
    }
}
