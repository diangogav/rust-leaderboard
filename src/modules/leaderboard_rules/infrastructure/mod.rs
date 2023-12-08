use crate::database::{self, models::mongo_leaderboard_rule::MongoLeaderboardRule};
use database::mongo_db::MongoDB;
use mongodb::bson::doc;
use rocket::futures::StreamExt;

use super::domain::{LeaderboardRule, LeaderboardRuleRepository};

pub struct MongodbLeaderboardRuleRepository<'a> {
    pub connection: &'a MongoDB,
}

impl MongodbLeaderboardRuleRepository<'_> {
    fn get_collection(&self) -> mongodb::Collection<MongoLeaderboardRule> {
        self.connection
            .database
            .collection::<MongoLeaderboardRule>("leaderboard_rules")
    }
}

#[async_trait]
impl LeaderboardRuleRepository for MongodbLeaderboardRuleRepository<'_> {
    async fn get(&self) -> Vec<LeaderboardRule> {
        let filter = doc! {};
        let options = mongodb::options::FindOptions::builder().build();

        let mut cursor = self.get_collection().find(filter, options).await.unwrap();
        let mut leaderboard_rules = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let leaderbaord_rule = LeaderboardRule::create(
                        document.min_position,
                        document.max_position,
                        document.score_per_win,
                        document.score_per_defeat,
                        document.bonus_per_win,
                        document.punishment_per_defeat,
                        document.name,
                    );

                    leaderboard_rules.push(leaderbaord_rule);
                }
                Err(_error) => {}
            }
        }

        leaderboard_rules
    }

    async fn find_by_position(&self, position: i32) -> Option<LeaderboardRule> {
        let filter: mongodb::bson::Document = doc! {
            "min_position": { "$lte": position },
            "max_position": { "$gte": position }
        };
        let options = mongodb::options::FindOneOptions::builder().build();

        if let Ok(result) = self.get_collection().find_one(filter, options).await {
            if let Some(document) = result {
                return Some(LeaderboardRule::create(
                    document.min_position,
                    document.max_position,
                    document.score_per_win,
                    document.score_per_defeat,
                    document.bonus_per_win,
                    document.punishment_per_defeat,
                    document.name,
                ));
            } else {
                return None;
            }
        }

        return None;
    }
}
