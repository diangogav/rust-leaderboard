use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoMessage {
    pub _id: ObjectId,

    pub content: String,
}
