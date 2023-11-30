use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoLeaderboard {
    pub id: String,

    pub name: String,
}
