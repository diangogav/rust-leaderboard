use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoLeaderboardRule {
    pub min_position: i32,

    pub max_position: i32,

    pub score_per_win: i32,

    pub score_per_defeat: i32,

    pub bonus_per_win: i32,

    pub punishment_per_defeat: i32,

    pub name: String,
}
