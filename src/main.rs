extern crate diesel;

use dotenvy::dotenv;
use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use self::models::{loans, users};

pub mod schema;
pub mod models;


fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set ");
    let mut connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // println!("connection successful!");

    use crate::users::NewUser;
    let new_user = NewUser {
        name: "Papa Wemba",
        email: "papawemba@gmail.com"
    };

    diesel::insert_into(schema::users::dsl::users)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error saving new user");

        use crate::loans::NewLoan;
    let new_loan = NewLoan {
        user_id: 4,
        amount: 458.13,
    }; 

    diesel::insert_into(schema::loans::dsl::loans)
        .values(&new_loan)
        .execute(&mut connection)
        .expect("Error saving new loan");


}
