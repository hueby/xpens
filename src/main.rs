extern crate chrono;

use chrono::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Money {
    item: String,
    euros: i32,
    cents: i32,
    date: DateTime<Local>,
}


fn main() {

    type Income = Money;
    type Outcome = Money;

    let mut incomes = HashSet::new();
    let mut outcomes = HashSet::new();

    for x in 0..3 {
        let test_inc = Income {
            item: String::from("Test"),
            euros: 7,
            cents: 40,
            date: Local::now(),
        };
        println!("{:?}", &test_inc);
        incomes.insert(test_inc);
    }

    for x in 0..3 {
        let test_ouc = Outcome {
            item: String::from("MORE MONEY"),
            euros: 10000,
            cents: 0,
            date: Local::now(),
        };
        println!("{:?}", &test_ouc);
        outcomes.insert(test_ouc);
    }


}
