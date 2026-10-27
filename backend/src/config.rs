use mongodb::{bson::doc, Client, Collection};

use crate::models::auth::Auth;

pub async fn connect_db() -> Result<Collection<Auth>, mongodb::error::Error> {
    let uri = std::env::var("MONGODB_URI")
        .expect("MONGODB_URI est manquant dans le fichier .env");

    if uri.contains("<password>") || uri.contains("<db_password>") {
        panic!("MONGODB_URI contient encore <password> : remplace-le par le vrai mot de passe dans .env");
    }

    let db_name = std::env::var("DATABASE_NAME")
        .unwrap_or_else(|_| "tickets_management".to_string());

    let client = Client::with_uri_str(&uri).await?;

    if let Err(error) = client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await
    {
        if error.to_string().contains("Authentication failed") {
            eprintln!("Identifiant ou mot de passe incorrect dans MONGODB_URI (fichier .env).");
        }
        return Err(error);
    }

    println!("MongoDB connection successful!");

    let collection = client.database(&db_name).collection::<Auth>("users");

    Ok(collection)
}