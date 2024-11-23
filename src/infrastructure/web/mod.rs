use axum::Router;

use crate::presentation::routes::{record_routes, store_routes};

/// initializes the http web server
pub async fn run() -> Result<(), String> {
    let store_routes = store_routes::routes();
    let record_routes = record_routes::routes();

    println!("Starting web server...!");

    // build our application with a route
    let app = Router::new().merge(store_routes).merge(record_routes);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
