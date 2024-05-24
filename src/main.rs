mod transaction;
mod transaction_service;

#[macro_use] extern crate rocket;

use mongodb::bson::oid::ObjectId;
use crate::transaction::Transaction;
use crate::transaction_service::TransactionService;



#[get("/postTransaction")]
async fn index() -> String {
    let service = TransactionService::new().expect("Failed to get mongodb uri.");
    let test_transaction = Transaction::new(
        ObjectId::new(),
        ObjectId::new(),
        ObjectId::new(),
        200,
        "now".to_string(),
        "fake description".to_string()
    );
    
    let new_id = service.post_transaction(test_transaction).await.unwrap();
    format!("new transaction id: {}", new_id)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
