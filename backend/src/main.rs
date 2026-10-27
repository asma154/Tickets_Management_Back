mod config;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    match config::connect_db().await {
        Ok(_collection) => {
            println!("Authentication API: database ready!");
        }
        Err(error) => {
            eprintln!("MongoDB connection failed: {error}");
            std::process::exit(1);
        }
    }
}