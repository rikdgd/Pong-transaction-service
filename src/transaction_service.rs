use std::error::Error;
use std::env;
use std::env::VarError;
use crate::transaction::Transaction;
use mongodb::{
    bson::doc,
    bson::oid::ObjectId,
    Client,
    Collection
};



const DB_NAME: &str = "development";
const COLLECTION_NAME: &str = "transactions";



pub struct TransactionService {
    conn_string: String,
}
impl TransactionService {
    pub fn new() -> Result<Self, VarError> {
        Ok(
            TransactionService {
                conn_string: env::var("MONGODB_URI")?,
            }
        )
    }


    pub async fn get_transaction(&self, id: ObjectId) -> Result<Transaction, Box<dyn Error>> {
        let client = Client::with_uri_str(&self.conn_string).await?;

        let transaction_collection: Collection<Transaction> = client
            .database(DB_NAME)
            .collection(COLLECTION_NAME);

        let stringified_id = id.to_string();
        let res = transaction_collection.find_one(
            doc! { "_id": stringified_id },
            None
        ).await?;

        match res {
            Some(transaction) => Ok(transaction),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Transaction with id {} not found", id),
            ))),
        }
    }


    /// Used to create a new transaction in the database, 
    /// returns the _id of the newly created transaction if successful. 
    pub async fn post_transaction(&self, transaction: Transaction) -> Result<String, Box<dyn Error>> {
        let client = Client::with_uri_str(&self.conn_string).await?;

        let transaction_collection: Collection<Transaction> = client
            .database(DB_NAME)
            .collection(COLLECTION_NAME);

        let res = transaction_collection.insert_one(transaction, None).await?;
        
        Ok(res.inserted_id.to_string())
    }


    pub fn get_user_balance(&self, user_id: ObjectId) -> i64 {
        todo!()
    }
}
