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
    // N1
    let mut db: DB<&str, &str> = DB::new();
    db.put("hello", "world");
    db.put("ayep", "mommy");
    db.put("fuck", "you");
    db.put("hello", "ajameti");

    print!("{:?}\n{}\n{}\n", db, db.get("hello").unwrap(), db.get("fuck").unwrap());
}


fn main() {
    // call example
    example();
}
