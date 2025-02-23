use krabby_ai;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    krabby_ai::run_chat().await
}
