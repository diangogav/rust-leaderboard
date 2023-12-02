use rocket::http::Status;

use crate::modules::leaderboard::domain::{Leaderboard, LeaderboardRepository};

pub struct LeaderboardCreator;

impl LeaderboardCreator {
    pub async fn execute(
        &self,
        repository: impl LeaderboardRepository,
        id: String,
        name: String,
    ) -> Result<(), Status> {
        let leaderboard = Leaderboard::create(id.clone(), name.clone());
        if let Err(error) = repository.save(leaderboard).await {
            return Err(error);
        }

        Ok(())
    }
}
#[cfg(test)]

mod test {
    use rocket::tokio;

    use crate::modules::leaderboard::{
        application::leaderboard_creator::LeaderboardCreator, domain::MockLeaderboardRepository,
    };

    #[tokio::test]
    async fn create_leaderboard_success() {
        // Arrange
        let repository = MockLeaderboardRepository::new();
        // Act
        let leaderboard_creator = LeaderboardCreator {};
        let result = leaderboard_creator
            .execute(repository, "id".to_string(), "name".to_string())
            .await;

        // Assert
        // assert!(result.is_ok());
    }
}
