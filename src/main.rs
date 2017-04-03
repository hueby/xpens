extern crate chrono;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

use chrono::prelude::*;
use std::collections::HashSet;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::{FindOptions, FindOneAndUpdateOptions, IndexModel, IndexOptions,
                             ReturnDocument};


#[derive(Debug, Hash, Eq, PartialEq)]
struct Money {
    item: String,
    euros: i32,
    cents: i32,
    date: DateTime<Local>,
}

fn main() {

    let client = Client::connect("localhost", 27042).ok().expect("Failed");
    let db = client.db("test");

    let coll = db.list_collections(None).ok().expect("Authorized?");

    type Income = Money;
    type Outcome = Money;

    let mut incomes = HashSet::new();
    let mut outcomes = HashSet::new();

    for x in 0..3 {
        let test_inc = Income {
            item: String::from("Test"),
            euros: 7 * x,
            cents: 40,
            date: Local::now(),
        };
        println!("{:?}", &test_inc);
        incomes.insert(test_inc);
    }

    for x in 0..3 {
        let test_ouc = Outcome {
            item: String::from("MORE MONEY"),
            euros: 10000 * x,
            cents: 0,
            date: Local::now(),
        };
        println!("{:?}", &test_ouc);
        outcomes.insert(test_ouc);
    }
}
