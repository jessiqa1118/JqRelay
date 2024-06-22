mod server;

use tokio;

#[tokio::main]
async fn main() {
    server::start().await;
}
