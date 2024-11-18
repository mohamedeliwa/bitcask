use crate::infrastructure::repositories::postgres_store_repo::PostgresStoreRepo;

pub fn run() -> Result<(), String> {
    let _repo = PostgresStoreRepo::new();

    println!("Starting...!");

    // initializing the http web server

    Ok(())
}
