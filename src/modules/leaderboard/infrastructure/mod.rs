use crate::database;
use database::models::mongo_leaderboard::MongoLeaderboard;
use database::mongo_db::MongoDB;
use mongodb::bson::doc;
use rocket::http::Status;

use super::domain::{Leaderboard, LeaderboardRepository};

pub struct MongodbLeaderboardRepository<'a> {
    pub connection: &'a MongoDB,
}

impl MongodbLeaderboardRepository<'_> {
    fn get_collection(&self) -> mongodb::Collection<MongoLeaderboard> {
        self.connection
            .database
            .collection::<MongoLeaderboard>("leaderboards")
    }
}

#[async_trait]
impl LeaderboardRepository for MongodbLeaderboardRepository<'_> {
    async fn save(&self, leaderboard: Leaderboard) -> Result<String, Status> {
        let new_leaderboard = MongoLeaderboard {
            id: leaderboard.id,
            name: leaderboard.name,
        };

        return match self
            .get_collection()
            .insert_one(&new_leaderboard, None)
            .await
        {
            Ok(_) => Ok(new_leaderboard.id.to_string()),
            Err(_) => Err(Status::InternalServerError),
        };
    }

    async fn find(&self, name: String) -> Option<Leaderboard> {
        let filter = doc! { "name": name };
        let options = mongodb::options::FindOneOptions::builder().build();

        if let Ok(result) = self.get_collection().find_one(filter, options).await {
            if let Some(document) = result {
                return Some(Leaderboard::create(document.id, document.name));
            } else {
                return None;
            }
        }

        return None;
    }
}
