use crate::database::mongo_db::MongoDB;
use crate::modules::leaderboard::domain::LeaderboardRepository;
use crate::modules::leaderboard::infrastructure::MongodbLeaderboardRepository;
use crate::modules::leaderboard_user::domain::LeaderboardUserRepository;
use crate::modules::leaderboard_user::infrastructure::MongodbLeaderboardUserRepository;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct GetLeaderboardUserPositionResponse {
    data: u64,
}

#[openapi(tag = "Leaderboard")]
#[get("/<leaderboard_name>/user/<username>/position")]
pub async fn handle(
    connection: &State<MongoDB>,
    leaderboard_name: String,
    username: String,
) -> Result<Json<GetLeaderboardUserPositionResponse>, Status> {
    let leaderboard_user_repository = MongodbLeaderboardUserRepository { connection };
    let leaderboard_repository = MongodbLeaderboardRepository { connection };

    let leaderboard_option = leaderboard_repository.find(leaderboard_name).await;

    if let None = leaderboard_option {
        return Err(Status::NotFound);
    }

    let leaderboard = leaderboard_option.unwrap();

    let leaderboard_user_option = leaderboard_user_repository
        .find(username, leaderboard.id)
        .await;

    if let None = leaderboard_user_option {
        return Err(Status::NotFound);
    }

    let position_option = leaderboard_user_repository
        .get_position(leaderboard_user_option.unwrap())
        .await;

    if let None = position_option {
        return Err(Status::NotFound);
    }

    let response = GetLeaderboardUserPositionResponse {
        data: position_option.unwrap(),
    };

    Ok(Json(response))
}
