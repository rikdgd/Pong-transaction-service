mod transaction;
mod transaction_service;
mod transaction_post_model;

#[macro_use] extern crate rocket;

use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use crate::transaction::Transaction;
use crate::transaction_post_model::TransactionPostModel;
use crate::transaction_service::TransactionService;


#[get("/test")]
async fn test_endpoint() -> &'static str {
    "Hello world!"
}


#[get("/getTransaction/<transaction_id>")]
async fn get_transaction(transaction_id: String) -> String {
    let service = TransactionService::new().expect("Failed to get mongodb uri.");
    
    let obj_id = ObjectId::parse_str(transaction_id).expect("Failed to convert user input to ObjectId");
    let res = service.get_transaction(obj_id).await.expect("Failed to get a transaction with the given id.");
    
    format!("{:?}", res)
}


#[get("/postTestTransaction")]
async fn post_test_transaction() -> String {
    let service = TransactionService::new().expect("Failed to get mongodb uri.");
    let test_transaction = Transaction::new(
        Some(ObjectId::new()),
        ObjectId::new(),
        ObjectId::new(),
        200,
        "fake description".to_string()
    );

    let new_id = service.post_transaction(test_transaction).await.unwrap();
    format!("new transaction id: {}", new_id)
}


#[post("/createTransaction", data = "<transaction>")]
async fn post_transaction(transaction: Json<TransactionPostModel>) -> String {
    let service = TransactionService::new().expect("Failed to get mongodb uri.");
    
    let transaction_post_model = transaction.into_inner();
    let parsed_transaction = transaction_post_model.to_transaction();
    
    let new_id = service.post_transaction(parsed_transaction).await
        .expect("Failed to post transaction based on given data.");
    format!("New transaction id: {}", new_id)
}


#[get("/getUserBalance/<user_id>")]
async fn get_user_balance(user_id: String) -> String {
    let service = TransactionService::new().expect("Failed to get mongodb uri.");
    let balance = service.get_user_balance(ObjectId::parse_str(user_id).unwrap()).await;

    format!("{}", balance)
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        test_endpoint,
        get_transaction,
        post_test_transaction,
        post_transaction,
        get_user_balance,
    ])
}
