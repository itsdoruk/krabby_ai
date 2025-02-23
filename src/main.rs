mod client;
mod config;
mod models;
mod utils;

use models::message::Message;
use std::io::{self, Write};
use colored::*;
use utils::display_markdown;
use whoami;
use hostname;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let client = client::Client::new();

    let username = whoami::username();
    let hostname = hostname::get()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    println!("{}", "AI Chat (type 'exit' to quit)".bright_green());
    loop {
        print!("{}", format!("{}@{}: ", username, hostname).bright_blue());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let message = Message {
            role: "user".to_string(),
            content: input.to_string(),
        };

        match client.send_message(message).await {
            Ok(response) => {
                println!("{}", "AI: ".bright_green());
                display_markdown(&response);
            },
            Err(e) => eprintln!("{}", format!("Error: {}", e).bright_red()),
        }
        println!();
    }
}