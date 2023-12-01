use rocket::http::Status;

#[derive(Clone)]
pub struct Leaderboard {
    pub id: String,
    pub name: String,
}

impl Leaderboard {
    pub fn create(id: String, name: String) -> Self {
        Leaderboard { id, name }
    }
}

#[async_trait]
pub trait LeaderboardRepository {
    async fn save(&self, leaderboard: Leaderboard) -> Result<String, Status>;
    async fn find(&self, name: String) -> Option<Leaderboard>;
}
