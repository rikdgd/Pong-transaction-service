use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;



#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub sender_id: ObjectId,
    pub receiver_id: ObjectId,
    pub amount: i64,
    pub time_stamp: String,
    pub description: String,
}
impl Transaction {
    pub fn new(
        id: Option<ObjectId>,
        sender_id: ObjectId,
        receiver_id: ObjectId,
        amount: i64,
        description: String
    ) -> Self {
        let utc: DateTime<Utc> = Utc::now();
        let time_stamp = utc.to_string();

        Self { id, sender_id, receiver_id, amount, time_stamp, description }
    }
}
