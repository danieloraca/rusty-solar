mod cli;

use crate::cli::cli;

#[tokio::main]
async fn main() {
    cli().await;
}
