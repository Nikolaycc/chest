#[allow(unused_imports)]
use std::ops::Add;

#[macro_export]
macro_rules! chsdb {
    ($keyty:ty: $key:expr => $valuety:ty: $value:expr) => {
        {
            let mut tmp_chsdb: DB<$valuety, $keyty> = DB::new();

            tmp_chsdb.put($key, $value);

            tmp_chsdb
        }
    };
}

#[macro_export]
macro_rules! s {
    ($s:expr) => {
        $s.to_owned()
    };
}

pub fn db() -> std::io::Result<usize> {
    todo!();
}

#[derive(Debug)]
pub struct DB<T = String, K = String> {
    pub path: Option<String>,
    pub db: Option<usize>,
    pub values: Vec<T>,
    pub keys: Vec<K>,
}

impl<T: Default, K: Default> DB<T, K> {
    pub fn new() -> Self {
        Self {
            path: None,
            db: None,
            values: vec![],
            keys: vec![],
        }
    }

    pub fn put(&mut self, key: K, val: T) -> std::io::Result<()> {
        self.keys.push(key);
        self.values.push(val);

        Ok(())
    }
}
