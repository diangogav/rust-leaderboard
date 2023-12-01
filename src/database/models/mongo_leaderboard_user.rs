use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoLeaderboardUser {
    pub id: String,

    pub leaderboard_id: String,

    pub username: String,

    pub wins: i32,

    pub losses: i32,

    pub points: i32,
}
