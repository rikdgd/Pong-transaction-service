use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    id: ObjectId,
    pub sender_id: ObjectId,
    pub receiver_id: ObjectId,
    pub amount: i64,
    pub time_stamp: String,
    pub description: String,
}
impl Transaction {
    pub fn new(
        id: ObjectId,
        sender_id: ObjectId,
        receiver_id: ObjectId,
        amount: i64,
        time_stamp: String,
        description: String
    ) -> Self {
        Self { id, sender_id, receiver_id, amount, time_stamp, description }
    }
    
    pub fn get_id(&self) -> ObjectId {
        self.id
    }
}