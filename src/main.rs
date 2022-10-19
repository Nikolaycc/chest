#[cfg(test)]
mod tests;

mod chsdb;

#[allow(unused_imports)]
use std::io;
use chsdb::*;

#[allow(dead_code)]
fn run_process() -> io::Result<()> {
    todo!();
}

fn example() -> () {
    // 1. Create DB & Put (key => value)
    let mut db: DB<&str, &str> = DB::new();
    db.put("hello", "world").unwrap();
    db.put("ayep", "mommy").unwrap();
    db.put("fuck", "you").unwrap();
    db.put("hello", "ajameti").unwrap();

    print!("{:?}\n{}\n{}\n{}\n", db, db.get("hello").unwrap(), db.get("fuck").unwrap(), db.len().unwrap());
}


fn main() {
    // call example code
    example();
}
