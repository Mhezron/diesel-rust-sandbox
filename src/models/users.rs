// use super::schema::loans;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String
}


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]

pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}
