use krabby_ai;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    krabby-ai::run_chat().await
}
