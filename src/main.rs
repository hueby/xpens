extern crate chrono;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

use chrono::prelude::*;
use std::collections::HashSet;
use std::env;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::{FindOneAndUpdateOptions, IndexModel, IndexOptions, ReturnDocument};
use bson::Bson;

fn main() {

    let mut client: Client = connect_to_db();

    #[derive(Debug, Hash, Eq, PartialEq)]
    struct Money {
        item: String,
        euros: i32,
        cents: i32,
        date: DateTime<Local>,
    }

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

fn connect_to_db() -> Client {
    let key = "MONGOPORT";
    let client = match env::var(key) {
        Ok(val) => Client::connect("localhost", val.parse::<u16>().unwrap()).ok().expect("Failed"),
        _ => Client::connect("localhost", 27017).ok().expect("Failed"),
    };

    let coll = client.db("test").collection("test");

    let mut cursor = coll.find(None, None).unwrap();

    let item = cursor.next();
    match item {
        Some(Ok(doc)) => {
            match doc.get("x") {
                Some(&Bson::I32(ref title)) => println!("{}", title),
                _ => panic!("Expected title to be a string!"),
            }
        }
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }
    client
}
