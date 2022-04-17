use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use crate::store::Store;

#[derive(Debug)]
pub struct HashMapStore {
    store: RefCell<HashMap<u32, Vec<u8>>>,
}

impl Default for HashMapStore {
    fn default() -> Self {
        HashMapStore { store: RefCell::new(HashMap::<u32, Vec<u8>>::new()) }
    }
}

impl Store for HashMapStore {
    fn put(&self, hash: u32, value: Vec<u8>) {
        self.store.borrow_mut().insert(hash, value);
    }

    fn get(&self, hash: &u32) -> Option<Vec<u8>> {
        let borrowed_store = self.store.borrow();
        let value = borrowed_store.get(hash);
        value.map(|val| val.to_vec())
    }
}

#[cfg(test)]
#[allow(warnings, unused)]
mod tests {
    use crate::DValue::Integer;
    use super::{HashMapStore, Store};

    #[test]
    fn test_push_to_store() {
        let mut hstore = HashMapStore::default();
        let key1 = 123 as u32;
        let key2 = 1234 as u32;
        let key3 = 12345 as u32;

        hstore.put(key1, "hello".as_bytes().to_vec());
        hstore.put(key2, (1234 as i32).to_le_bytes().to_vec());
        hstore.put(key3, Integer(12345).into());
        let val1 = hstore.get(&key1).unwrap();
        let val2 = hstore.get(&key2).unwrap();
        let val3 = hstore.get(&key3).unwrap();


        assert_eq!("hello", String::from_utf8(val1.clone()).unwrap());
        assert_eq!(1234 as i32, i32::from_le_bytes(val2.try_into().unwrap()));
        assert_eq!(12345_i32.to_le_bytes().to_vec(), val3);
    }
}