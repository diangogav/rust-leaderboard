use mongodb;
use mongodb::bson;
use mongodb::Client;
use mongodb::Database;
use std::env;

const DATABASE: &str = "leaderboard";

pub struct MongoDB {
    pub database: Database,
}

impl MongoDB {
    fn new(database: Database) -> Self {
        MongoDB { database }
    }
}

pub async fn connect() -> mongodb::error::Result<MongoDB> {
    let mongo_db_uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI required");

    let client = Client::with_uri_str(mongo_db_uri).await.unwrap();

    client
        .database("admin")
        .run_command(bson::doc! {"ping": 1}, None)
        .await?;

    Ok(MongoDB::new(client.database(DATABASE)))
}
