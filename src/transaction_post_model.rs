use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::transaction::Transaction;


#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionPostModel {
    pub sender_id: ObjectId,
    pub receiver_id: ObjectId,
    pub amount: i64,
    pub description: String,
}
impl TransactionPostModel {
    pub fn to_transaction(&self) -> Transaction {
        let utc: DateTime<Utc> = Utc::now();
        let time_stamp = utc.to_string();
        
        Transaction::new(
            None,
            self.sender_id,
            self.receiver_id,
            self.amount,
            time_stamp,
            self.description.clone(),
        )
    }
}
