use crate::database::mongo_db::MongoDB;
use crate::modules::leaderboard::domain::LeaderboardRepository;
use crate::modules::leaderboard::infrastructure::MongodbLeaderboardRepository;
use crate::modules::leaderboard_user::domain::{LeaderboardUser, LeaderboardUserRepository};
use crate::modules::leaderboard_user::infrastructure::MongodbLeaderboardUserRepository;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::State;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Player {
    username: String,
    winner: bool,
}
#[derive(Debug, Deserialize, JsonSchema)]

pub struct UpdateLeaderboardUserRequest {
    leaderboard_name: String,
    players: Vec<Player>,
}

#[derive(Serialize, JsonSchema)]
pub struct UpdateLeaderboardUserResponse {
    success: bool,
}

#[openapi(tag = "Leaderboard")]
#[post("/calculate", data = "<request>")]
pub async fn handle(
    connection: &State<MongoDB>,
    request: Json<UpdateLeaderboardUserRequest>,
) -> Result<Json<UpdateLeaderboardUserResponse>, Status> {
    let leaderboard_repository = MongodbLeaderboardRepository { connection };
    let leaderboard_user_repository = MongodbLeaderboardUserRepository { connection };

    let leaderboard_option = leaderboard_repository
        .find(request.leaderboard_name.to_string())
        .await;

    if let None = leaderboard_option {
        return Err(Status::NotFound);
    }

    let leaderboard = leaderboard_option.unwrap();

    for player in &request.players {
        let leaderboard_user_option = leaderboard_user_repository
            .find(player.username.to_string(), leaderboard.id.to_string())
            .await;

        if let None = leaderboard_user_option {
            let leaderboard_user = LeaderboardUser::create(
                Uuid::new_v4().to_string(),
                leaderboard.id.to_string(),
                player.username.to_string(),
                if player.winner { 1 } else { 0 },
                if player.winner { 0 } else { 1 },
                0,
            );
            if let Err(status) = leaderboard_user_repository.save(leaderboard_user).await {
                return Err(status);
            };

            continue;
        }

        let mut leaderboard_user = leaderboard_user_option.unwrap();

        if player.winner {
            leaderboard_user.increment_wins();
        } else {
            leaderboard_user.increment_losses();
        }

        if let Err(status) = leaderboard_user_repository.save(leaderboard_user).await {
            return Err(status);
        };
    }

    let response = UpdateLeaderboardUserResponse { success: true };
    Ok(Json(response))
}
