use diesel::prelude::*;
use chrono::NaiveDateTime;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::loans)]

pub struct Loan {
    pub amount: f64,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::loans)]

pub struct NewLoan {
    pub user_id: i32,
    pub amount:  f64,
}