mod exfor_client;

#[tokio::main]
async fn main() {
    let body = exfor_client::fetch_data("Mo-94", "n,g", "SIG").await;
    println!("body = {:?}", body.sections[0]);
}
