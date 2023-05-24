mod server;

use server::Result;

#[tokio::main]
async fn main() -> Result {
    server::serve().await
}
