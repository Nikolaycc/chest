use crate::*;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn check_keys_values() {
    let mut db: DB<&str, &str> = DB::new();
    db.put("hello", "world");
    db.put("ayeo", "mommy");
    db.put("fuck", "you");
    db.put("hello", "world");

    assert_eq!(db.get("hello").unwrap(), "world");
    assert_eq!(db.get("ayeo").unwrap(), "mommy");
    assert_eq!(db.get("fuck").unwrap(), "you");
}

#[test]
fn check_struct() {
    let db = chsdb!(&str: "Key1" => u32: 10);

    assert_eq!(db, chsdb!(&str: "Key1" => u32: 10));
}
