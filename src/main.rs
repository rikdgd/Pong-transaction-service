mod transaction;
mod transaction_service;

#[macro_use] extern crate rocket;

use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use crate::transaction::Transaction;
use crate::transaction_service::TransactionService;



#[get("/postTestTransaction")]
async fn index() -> String {
    let service = TransactionService::new().expect("Failed to get mongodb uri.");
    let test_transaction = Transaction::new(
        Some(ObjectId::new()),
        ObjectId::new(),
        ObjectId::new(),
        200,
        "now".to_string(),
        "fake description".to_string()
    );

    let new_id = service.post_transaction(test_transaction).await.unwrap();
    format!("new transaction id: {}", new_id)
}

#[post("/createTransaction", data = "<transaction>")]
async fn post_transaction(transaction: Json<Transaction>) -> String {
    let service = TransactionService::new().expect("Failed to get mongodb uri.");
    
    let mut parsed_transaction = transaction.into_inner();
    parsed_transaction.id = None;
    let new_id = service.post_transaction(parsed_transaction).await.unwrap();
    
    format!("New transaction id: {}", new_id)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, post_transaction])
}
