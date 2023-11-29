use rocket::http::Status;

#[derive(Clone)]
pub struct Message {
    pub content: String,
}

impl Message {
    pub fn create(content: String) -> Self {
        Message { content }
    }
}

#[async_trait]
pub trait MessageRepository {
    async fn save(&self, message: Message) -> Result<String, Status>;
}
