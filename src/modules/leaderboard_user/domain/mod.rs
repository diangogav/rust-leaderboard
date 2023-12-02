use rocket::http::Status;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::Serialize;

pub struct LeaderboardUser {
    pub id: String,
    pub leadboard_id: String,
    pub username: String,
    pub wins: i32,
    pub losses: i32,
    pub points: i32,
}
#[derive(Serialize, JsonSchema)]

pub struct LeaderboardUserDto {
    id: String,
    username: String,
    wins: i32,
    losses: i32,
    points: i32,
}

impl LeaderboardUser {
    pub fn create(
        id: String,
        leadboard_id: String,
        username: String,
        wins: i32,
        losses: i32,
        points: i32,
    ) -> Self {
        LeaderboardUser {
            id,
            leadboard_id,
            username,
            wins,
            losses,
            points,
        }
    }

    pub fn increment_wins(&mut self) {
        self.wins = self.wins + 1;
    }

    pub fn increment_losses(&mut self) {
        self.losses = self.losses + 1;
    }

    pub fn to_presentation(&self) -> LeaderboardUserDto {
        LeaderboardUserDto {
            id: self.id.clone(),
            username: self.username.clone(),
            wins: self.wins.clone(),
            losses: self.losses.clone(),
            points: self.points.clone(),
        }
    }
}

#[async_trait]
pub trait LeaderboardUserRepository {
    async fn save(&self, leaderboard_user: LeaderboardUser) -> Result<(), Status>;
    async fn find(&self, username: String, leaderboard_id: String) -> Option<LeaderboardUser>;
    async fn get(&self, leaderboard_id: String) -> Vec<LeaderboardUserDto>;
}
