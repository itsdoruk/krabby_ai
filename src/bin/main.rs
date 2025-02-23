use krabby;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    krabby::run_chat().await
}
