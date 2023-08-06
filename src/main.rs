#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    match clinicaltrials_rs::run().await {
        Ok(_) => tracing::info!("OK"),
        Err(e) => tracing::info!("{:#?}", e),
    };

    if cfg!(target_os = "windows") {
        tracing::info!("> Press [Enter] to close terminal <");
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }
}
