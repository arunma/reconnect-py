pub mod hashmap;

pub trait Store: Default {
    fn put (&self, hash: u32, value: Vec<u8>);
    fn get (&self, hash: &u32) -> Option<Vec<u8>>;
}

