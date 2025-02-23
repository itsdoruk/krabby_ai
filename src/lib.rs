pub mod client;
pub mod config;
pub mod models;
pub mod utils;

use models::message::Message;
use std::io::{self, Write};
use colored::*;

pub async fn run_chat() -> io::Result<()> {
    let client = client::Client::new();
    let mut conversation_history: Vec<Message> = Vec::new();

    let username = whoami::username();
    let hostname = hostname::get()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // Add system message to set the context
    conversation_history.push(Message {
        role: "system".to_string(),
        content: "You are a helpful AI assistant.".to_string(),
    });

    println!("{}", "AI Chat (type 'exit' to quit)".bright_green());
    loop {
        print!("{}", format!("{}@{}: ", username, hostname).bright_blue());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let user_message = Message {
            role: "user".to_string(),
            content: input.to_string(),
        };
        conversation_history.push(user_message);

        match client.send_message(conversation_history.clone()).await {
            Ok(response) => {
                println!("{}", "AI: ".bright_green());
                utils::display_markdown(&response);
                
                // Add AI's response to conversation history
                conversation_history.push(Message {
                    role: "assistant".to_string(),
                    content: response,
                });
            },
            Err(e) => eprintln!("{}", format!("Error: {}", e).bright_red()),
        }
        println!();
    }
    Ok(())
}
