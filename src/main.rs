use infrastructure::web;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[tokio::main]
async fn main() {
    // starting the web server
    web::run().await.unwrap();
}
