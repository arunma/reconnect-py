use std::io::Cursor;
use murmur3::murmur3_32;

// impl Default for DataHasher<[u8]> {
//     fn default() -> DataHasher<[u8]> {
//         MurmurHash::default()
//     }
// }

pub trait DataHasher: Default {
    //fn leaf<T: Into<Vec<u8>>>(&self, val: &T) -> u32;
    fn leaf(&self, val: &[u8]) -> u32;
    fn node(&self, left: u32, right: u32) -> u32;
}

#[derive(Default)]
pub struct MurmurHasher;

impl DataHasher for MurmurHasher {
    //fn leaf<T: Into<Vec<u8>>>(&self, val: &T) -> u32 {
    fn leaf(&self, val: &[u8]) -> u32 {
        murmur3_32(&mut Cursor::new(val), 0).unwrap()
    }

    fn node(&self, left: u32, right: u32) -> u32 {
        //let hash_vec:Vec<u32> = Vec::new();
        let mut hash_vec = Vec::new();
        hash_vec.extend(left.to_le_bytes());
        hash_vec.extend(right.to_le_bytes());
        murmur3_32(&mut Cursor::new(&hash_vec), 0).unwrap()
    }
}
