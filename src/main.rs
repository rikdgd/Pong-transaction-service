mod transaction;
mod transaction_service;
mod controller;
mod messaging;

#[macro_use] extern crate rocket;

use controller::*;



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        post_test_transaction,
        post_transaction,
        get_user_balance,
    ])
}
