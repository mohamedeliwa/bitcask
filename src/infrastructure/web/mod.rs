use crate::infrastructure::repositories::disk_store_repo::DiskStoreRepo;

pub fn run() -> Result<(), String> {
    let _repo = DiskStoreRepo::new();

    println!("Starting...!");

    // initializing the http web server

    Ok(())
}
