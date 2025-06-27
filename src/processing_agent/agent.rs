use std::env;

use genai::chat::printer::print_chat_stream;
use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;


const PROCESS_SYSTEM_CONFIGURATION="NOT YET INCLUDED";

dotenv_flow::dotenv_flow().ok();
Const MODEL: String = env::var("MODEL_ANTHROPIC").expect("The Model name must exist");

#[tokio::main]
pub async fn process(_text:&str) -> Option<std::string::String> {

    let client = Client::default();

    let chat_req: ChatRequest = ChatRequest::new(vec![
        ChatMessage::system(PROCESS_SYSTEM_CONFIGURATION),
        ChatMessage::user(_text.to_string())
    ]);

    let model: &str = MODEL;

    let chat_res = client.exec_chat_stream(model, chat_req, None).await.ok();
    
    let routing_response = match print_chat_stream(chat_res.expect("REASON"),  None).await {

        Ok(response) => {
            return Some(response);
        },

        Err(_) => {
            return Some("Error Processing".to_string());
        }
    };

    
}
