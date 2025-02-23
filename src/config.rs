
pub struct Config {
    pub api_endpoint: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            api_endpoint: String::from("https://ai.hackclub.com/chat/completions"),
        }
    }
}