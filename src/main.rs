// REMOVE THIS!!!!
#[allow(dead_code)]

mod db;
mod utils;

use db::DB;

fn main() {
    let mut database = DB::new();
    database.load()
}

