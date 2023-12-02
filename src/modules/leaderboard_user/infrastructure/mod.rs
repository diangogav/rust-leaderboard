use crate::database;
use database::models::mongo_leaderboard_user::MongoLeaderboardUser;
use database::mongo_db::MongoDB;
use mongodb::bson::doc;
use rocket::{futures::StreamExt, http::Status};

use super::domain::{LeaderboardUser, LeaderboardUserDto, LeaderboardUserRepository};

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

    async fn get(&self, leaderboard_id: String) -> Vec<LeaderboardUserDto> {
        let filter = doc! { "leaderboard_id": leaderboard_id };
        let options = mongodb::options::FindOptions::builder()
            .sort(doc! { "points": -1, "wins": -1 })
            .limit(50)
            .build();

        let mut cursor = self.get_collection().find(filter, options).await.unwrap();
        let mut leaderboard_users = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let leaderboard_user = LeaderboardUser::create(
                        document.id,
                        document.leaderboard_id,
                        document.username,
                        document.wins,
                        document.losses,
                        document.points,
                    );

                    leaderboard_users.push(leaderboard_user.to_presentation());
                }
                Err(_error) => {}
            }
        }

        leaderboard_users
    }
}
