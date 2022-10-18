#[allow(unused_imports)]
use std::ops::Add;

#[macro_export]
macro_rules! chsdb {
    ($keyty:ty: $key:expr => $valuety:ty: $value:expr) => {
        {
            let mut tmp_chsdb: DB<$valuety, $keyty> = DB::new();

            tmp_chsdb.put($key, $value).expect("Failed put key or value");

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

#[derive(Debug, Clone, PartialEq)]
pub struct DB<T = String, K = String> {
    pub path: String,
    pub db: usize,
    pub values: Vec<T>,
    pub keys: Vec<K>,
}

// impl<T: Default, K: Default> Copy for DB<T, K> { }

// impl<T: Default, K: Default> Clone for DB<T, K> {
//     fn clone(&self) -> DB {
//         DB {
//             path: self.path.clone(),
//             db: self.db,
//             values: self.values.clone(),
//             keys: self.keys.clone(),
//         }
//     }
// }

impl<T: Default + Clone, K: Default + std::cmp::PartialEq> DB<T, K> {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            db: 0,
            values: vec![],
            keys: vec![],
        }
    }

    pub fn put(&mut self, key: K, val: T) -> std::io::Result<()> {
        self.keys.push(key);
        self.values.push(val);

        Ok(())
    }

    pub fn get(&self, key: K) -> std::io::Result<T> {
        let index = self.keys.iter().position(|r| r == &key).unwrap();

        Ok(self.values[index].clone())
    }
}
