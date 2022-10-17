mod chsdb;

#[allow(unused_imports)]
use std::io;
use chsdb::*;

#[allow(dead_code)]
fn run_process() -> io::Result<()> {
    todo!();
}

fn main() {
    print!("Hello World\n");

    // db();

    let n1: DB<String, usize> = DB {
        path: Some("Hello World".to_owned()),
        db: Some(2424),
        values: vec![s!("Data"), s!("Data2")],
        keys: vec![24, 23, 54],
    };

    let n2 = DB::<u32, String>::new();

    let n3: DB<usize, String> = DB::new();

    let mut n4 = chsdb!(String: s!("key1") => String: s!("value1"));
    let n5 = chsdb!(i32: 2 => i32: 43);
    let n6 = chsdb!(usize: 53 => String: s!("BRUH"));

    n4.put(s!("key4"), s!("value4"));

    print!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n", n1, n2, n3, n4, n5, n6);
}
