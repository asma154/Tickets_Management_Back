use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub id_user: String,
    pub email: String,
    pub password: String,
    pub role_app: String,
}