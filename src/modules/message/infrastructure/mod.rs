use crate::database;
use database::models::mongo_message::MongoMessage;
use database::mongo_db::MongoDB;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;

use super::domain::{Message, MessageRepository};

pub struct MongodbMessageRepository<'a> {
    pub connection: &'a MongoDB,
}

impl MongodbMessageRepository<'_> {
    fn get_collection(&self) -> mongodb::Collection<MongoMessage> {
        self.connection
            .database
            .collection::<MongoMessage>("messages")
    }
}

#[async_trait]
impl MessageRepository for MongodbMessageRepository<'_> {
    async fn save(&self, message: Message) -> Result<String, Status> {
        let new_message = MongoMessage {
            _id: ObjectId::new(),
            content: message.content,
        };

        return match self.get_collection().insert_one(&new_message, None).await {
            Ok(_) => Ok(new_message._id.to_string()),
            Err(_) => Err(Status::InternalServerError),
        };
    }
}
