use std::fmt::Debug;
use anyhow::Result;
use crate::{Data};
use crate::hash::{DataHasher};
use crate::store::Store;

#[derive(Debug)]
pub struct MerkleTree<'a, S: Store, H: DataHasher> {
    pub hashes: Vec<u32>,
    pub store: &'a S,
    hasher: &'a H,
}

impl<'a, S: Store, H: DataHasher> MerkleTree<'a, S, H> {
    pub fn from_coll<I: IntoIterator<Item=impl Data>>(iter: I, store: &'a S, hasher: &'a H) -> Result<Self> {
        let into_iter = iter.into_iter();
        let mut hashes = match into_iter.size_hint().1 {
            Some(size) => {
                let next_pow = next_pow2(size);
                let data_size = 2 * next_pow - 1;
                Vec::with_capacity(data_size)
            }
            None => Vec::new()
        };

        for item in into_iter {
            let vec8 = item.into();
            let hash = hasher.leaf(&vec8);
            hashes.push(hash);
            store.put(hash, vec8);
        }

        //Pad data hashes
        let data_pad_length = next_pow2(hashes.len())-hashes.len();
        let padded_data = vec![0x00_u32; data_pad_length];
        hashes.extend(padded_data);

        let mut merkle = MerkleTree {
            hashes,
            store,
            hasher
        };

        merkle.build();

        Ok(merkle)
    }

    fn build(&mut self){
        let hasher = &self.hasher;
        let mut iter_width = self.hashes.len();

        let mut i:usize = 0;
        let mut j:usize = iter_width;

        while iter_width >1 {
            //Not needed anymore since the data is already padded to form a perfect binary tree.
          /*  if iter_width & 1 ==1 {
                let last_node = self.hashes[self.hashes.len()-1];
                self.hashes.push(last_node);
                iter_width+=1;
                j+=1;
            }*/

            while i<j{
                let hash = hasher.node(self.hashes[i], self.hashes[i+1]);
                self.hashes.push(hash);
                i+=2
            }

            iter_width >>= 1;
            j+=iter_width;
        }

        self.hashes.push(0x00); // Empty placeholder for starting index at 1
        self.hashes.reverse();
    }
}

pub fn next_pow2(mut v: usize) -> usize {
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v + 1
}


#[cfg(test)]
mod tests {
    use crate::DValue;
    use crate::DValue::*;
    use crate::hash::{MurmurHasher};
    use crate::merkle::{MerkleTree, next_pow2};
    use crate::store::hashmap::{HashMapStore};
    use crate::store::Store;

    #[test]
    fn test_build_merkle_tree_from_iterator() {
        //let data = [1, 2, 3, 4, 5];
        let expected_hashed_data: Vec<u32> = vec![0, 1946300346, 501029351, 194080189, 1669671676, 1957646064, 2389282197, 2340410264, 0, 0, 0, 108060675, 1889779975, 847579505, 4220927227, 613153351];
        let data: Vec<DValue> = vec![String("hello".to_string()), String("world".to_string()), Integer(3), Integer(4), Float(5.0)]; //.iter().map(|each|each.to_string()).collect();
        let store = HashMapStore::default();
        let hasher = MurmurHasher::default();
        let merkle = MerkleTree::from_coll(data, &store, &hasher).unwrap();
        println!("{:?}", &merkle.hashes);

        assert_eq!(merkle.hashes.len(), 16);
        assert_eq!(merkle.hashes, expected_hashed_data);
        assert_eq!(merkle.store.get(&613153351).unwrap(), "hello".as_bytes().to_vec())
    }

    #[test]
    fn test_next_pow2() {
        assert_eq!(next_pow2(6), 8);
        assert_eq!(next_pow2(7), 8);
        assert_eq!(next_pow2(8), 8);
    }
}