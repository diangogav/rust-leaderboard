use crate::database::mongo_db::MongoDB;
use crate::modules::leaderboard::domain::LeaderboardRepository;
use crate::modules::leaderboard::infrastructure::MongodbLeaderboardRepository;
use crate::modules::leaderboard_user::domain::{LeaderboardUserDto, LeaderboardUserRepository};
use crate::modules::leaderboard_user::infrastructure::MongodbLeaderboardUserRepository;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct GetLeaderboardUsersResponse {
    data: Vec<LeaderboardUserDto>,
}

#[openapi(tag = "Leaderboard")]
#[get("/<leaderboard_name>")]
pub async fn handle(
    connection: &State<MongoDB>,
    leaderboard_name: String,
) -> Result<Json<GetLeaderboardUsersResponse>, Status> {
    let leaderboard_user_repository = MongodbLeaderboardUserRepository { connection };
    let leaderboard_repository = MongodbLeaderboardRepository { connection };

    let leaderboard_option = leaderboard_repository.find(leaderboard_name).await;

    if let None = leaderboard_option {
        return Err(Status::NotFound);
    }

    let leaderboard = leaderboard_option.unwrap();

    let leaderboard_users = leaderboard_user_repository
        .get(leaderboard.id.to_string())
        .await;

    let response = GetLeaderboardUsersResponse {
        data: leaderboard_users,
    };
    Ok(Json(response))
}
