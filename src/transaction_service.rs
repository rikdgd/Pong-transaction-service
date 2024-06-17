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

        TransactionService::calculate_user_balance(user_id, user_transactions)
    }
    
    fn calculate_user_balance(user_id: ObjectId, transaction_list: UserTransactions) -> i64 {
        let mut balance = 0;
        
        for received in transaction_list.received {
            if received.receiver_id == user_id {
                balance += received.amount;
            }
        }
        for send in transaction_list.send {
            if send.sender_id == user_id {
                balance -= send.amount;
            }
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


#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;
    use mongodb::bson::oid::ObjectId;
    use crate::transaction_service::{TransactionService, UserTransactions};

    #[test]
    fn test_calculate_balance() {
        let user_a_id = ObjectId::new();
        let user_b_id = ObjectId::new();
        
        let send_transactions = vec![
            Transaction::new(None, user_a_id, user_b_id, 30, "now".to_string(), String::new()),
            Transaction::new(None, user_a_id, user_b_id, 70, "now".to_string(), String::new()),
        ];

        let received_transactions = vec![
            Transaction::new(None, user_b_id, user_a_id, 20, "now".to_string(), String::new()),
            Transaction::new(None, user_b_id, user_a_id, 50, "now".to_string(), String::new()),

            // Completely unrelated transaction, would normally never even arrive here
            Transaction::new(None, ObjectId::new(), ObjectId::new(), 20, "now".to_string(), String::new()),
        ];
        let user_transactions = UserTransactions::new(send_transactions, received_transactions);
        
        let expected_balance_user_a: i64 = -30;  // balance = -30 - 70 + 20 + 50
        
        
        let calculated_balance = TransactionService::calculate_user_balance(user_a_id, user_transactions);
        
        assert_eq!(expected_balance_user_a, calculated_balance);
    }
}
