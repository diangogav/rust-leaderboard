use crate::database;
use database::models::mongo_leaderboard_user::MongoLeaderboardUser;
use database::mongo_db::MongoDB;
use mongodb::bson::doc;
use rocket::http::Status;

use super::domain::{LeaderboardUser, LeaderboardUserRepository};

pub struct MongodbLeaderboardUserRepository<'a> {
    pub connection: &'a MongoDB,
}

impl MongodbLeaderboardUserRepository<'_> {
    fn get_collection(&self) -> mongodb::Collection<MongoLeaderboardUser> {
        self.connection
            .database
            .collection::<MongoLeaderboardUser>("leaderboard_users")
    }
}

#[async_trait]
impl LeaderboardUserRepository for MongodbLeaderboardUserRepository<'_> {
    async fn save(&self, leaderboard_user: LeaderboardUser) -> Result<(), Status> {
        let filter = doc! {"id": leaderboard_user.id };

        let update = doc! {"$set": {
            "username": leaderboard_user.username,
            "leaderboard_id": leaderboard_user.leadboard_id,
            "wins": leaderboard_user.wins,
            "losses": leaderboard_user.losses,
            "points":  leaderboard_user.points
        }};

        let options = mongodb::options::FindOneAndUpdateOptions::builder()
            .upsert(true)
            .build();

        return match self
            .get_collection()
            .find_one_and_update(filter, update, options)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(Status::InternalServerError),
        };
    }

    async fn find(&self, username: String, leaderboard_id: String) -> Option<LeaderboardUser> {
        let filter = doc! { "username": username, "leaderboard_id": leaderboard_id };
        let options = mongodb::options::FindOneOptions::builder().build();

        if let Ok(result) = self.get_collection().find_one(filter, options).await {
            if let Some(document) = result {
                return Some(LeaderboardUser::create(
                    document.id,
                    document.leaderboard_id,
                    document.username,
                    document.wins,
                    document.losses,
                    document.points,
                ));
            } else {
                return None;
            }
        }

        return None;
    }
}
