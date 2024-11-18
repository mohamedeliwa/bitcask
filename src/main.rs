use infrastructure::web;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

fn main() {
    println!("Hello, world!");
    // starting the web server
    web::run().unwrap();
}
