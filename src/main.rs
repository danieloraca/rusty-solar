mod cli;

// use crate::cli::cli;
use crate::cli::cli_menu;

#[tokio::main]
async fn main() {
    // cli().await;
    cli_menu().await;
}
