#[tokio::main]
async fn main() {
    match clinicaltrials_rs::run().await {
        Ok(_) => println!("OK"),
        Err(e) => println!("{:#?}", e),
    };

    println!("> Press [Enter] to close terminal....");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
