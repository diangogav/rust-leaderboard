pub struct LeaderboardRule {
    pub min_position: i32,
    pub max_position: i32,
    pub score_per_win: i32,
    pub score_per_defeat: i32,
    pub bonus_per_win: i32,
    pub punishment_per_defeat: i32,
    pub name: String,
}

impl LeaderboardRule {
    pub fn create(
        min_position: i32,
        max_position: i32,
        score_per_win: i32,
        score_per_defeat: i32,
        bonus_per_win: i32,
        punishment_per_defeat: i32,
        name: String,
    ) -> Self {
        LeaderboardRule {
            min_position,
            max_position,
            score_per_win,
            score_per_defeat,
            bonus_per_win,
            punishment_per_defeat,
            name,
        }
    }
}

#[async_trait]
pub trait LeaderboardRuleRepository {
    async fn get(&self) -> Vec<LeaderboardRule>;
    async fn find_by_position(&self, position: i32) -> Option<LeaderboardRule>;
}
