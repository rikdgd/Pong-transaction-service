use mongodb::{
    bson::oid::ObjectId,
    bson::doc,
    Client,
    Collection
};
use rocket::time::format_description::well_known::Iso8601;



pub struct Transaction {
    id: ObjectId,
    pub user_id: ObjectId,
    pub amount: u64,
    pub time_stamp: Iso8601,
    pub description: String,
}
impl Transaction {
    pub fn new(
        id: ObjectId,
        user_id: ObjectId,
        amount: u64,
        time_stamp: Iso8601,
        description: String
    ) -> Self {
        Self { id, user_id, amount, time_stamp, description }
    }
}