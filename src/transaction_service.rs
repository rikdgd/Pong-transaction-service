use std::error::Error;
use std::env;
use std::env::VarError;
use futures::TryStreamExt;
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

        let res = transaction_collection.find_one(
            doc! { "_id": id },
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


    pub async fn get_user_balance(&self, user_id: ObjectId) -> i64 {
        let user_transactions = self.get_user_involved_transactions(user_id)
            .await
            .expect("Failed to get user transactions.");
        
        let mut balance = 0;
        for received in user_transactions.received {
            balance += received.amount;
        }
        for send in user_transactions.send {
            balance -= send.amount;
        }
        
        balance
    }

    async fn get_user_involved_transactions(&self, user_id: ObjectId) -> Result<UserTransactions, Box<dyn Error>> {
        let mut user_transactions = UserTransactions::new(Vec::new(), Vec::new());
        let client = Client::with_uri_str(&self.conn_string).await?;

        let transaction_collection: Collection<Transaction> = client
            .database(DB_NAME)
            .collection(COLLECTION_NAME);


        let mut send_cursor = transaction_collection.find(
            doc! {"sender_id": user_id},
            None,
        ).await?;

        let mut received_cursor = transaction_collection.find(
            doc! {"receiver_id": user_id},
            None,
        ).await?;


        while let Some(doc) = send_cursor.try_next().await? {
            user_transactions.send.push(doc);
        }

        while let Some(doc) = received_cursor.try_next().await? {
            user_transactions.received.push(doc);
        }


        Ok(user_transactions)
    }
}

struct UserTransactions {
    pub send: Vec<Transaction>,
    pub received: Vec<Transaction>
}
impl UserTransactions {
    pub fn new(send: Vec<Transaction>, received: Vec<Transaction>) -> Self {
        Self { send, received }
    }
}
