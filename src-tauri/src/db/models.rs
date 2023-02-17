use crate::schema::client;
use crate::schema::seller;
use crate::schema::product;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Debug)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub addresse: String,
}


#[derive(Insertable, Serialize, Debug, Clone)]
#[table_name = "clients"]
pub struct NewClient<'a> {
    pub name: &'a str,,
    pub phone: &'a str,,
    pub email: &'a str,,
    pub addresse: &'a str,,
}