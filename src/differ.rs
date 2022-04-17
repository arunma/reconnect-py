use serde_derive::Serialize;

use crate::{COLON_STR};
use crate::hash::DataHasher;
use crate::merkle::MerkleTree;
use crate::store::Store;

#[derive(Debug, Serialize, PartialEq)]
pub struct Difference {
    pub row_id: String,
    pub col_name: String,
    pub left_value: String,
    pub right_value: String,

}

/*#[derive(Debug)]
struct DiffResult {
    store: HashMap<String, Vec<Difference>>,
}*/

pub struct Differ;

impl Differ {
    pub fn get_differences<'a, S: Store, H: DataHasher>(store: &'a S, left: &MerkleTree<S, H>, right: &MerkleTree<S, H>) -> Vec<Difference> {
        let length = left.hashes.len();
        let leaf_index = length / 2;
        assert_eq!(length, right.hashes.len());

        let mut queue: Vec<usize> = Vec::new();
        let mut differences = Vec::new();
        let root_index = 1;
        queue.push(root_index);

        while !queue.is_empty() {
            let index = queue.pop().unwrap();
            if index<length && left.hashes[index] != right.hashes[index] {
                if index > leaf_index {
                    let lstring = String::from_utf8(store.get(&left.hashes[index]).unwrap()).unwrap();
                    let mut lsplit = lstring.split(COLON_STR);
                    let rowid = lsplit.next().unwrap();
                    let lcolumn = lsplit.next().unwrap();
                    let lvalue = lsplit.next().unwrap();

                    let rstring = String::from_utf8(store.get(&right.hashes[index]).unwrap()).unwrap();
                    let mut rsplit = rstring.split(COLON_STR);
                    rsplit.next(); //Skip rowid and column
                    rsplit.next();
                    let rvalue = rsplit.next().unwrap();
                    let difference = Difference {
                        row_id: rowid.to_string(),
                        left_value: lvalue.to_string(),
                        right_value: rvalue.to_string(),
                        col_name: lcolumn.to_string(),
                    };
                    differences.push(difference)
                } else {
                    let half = index * 2;
                    queue.push(half);
                    queue.push(half + 1);
                }
            }
        }
        differences
    }

}


#[cfg(test)]
mod tests {
    use crate::{Column};
    use crate::differ::{Differ, Difference};
    use crate::hash::MurmurHasher;
    use crate::merkle::MerkleTree;
    use crate::store::hashmap::HashMapStore;

    #[test]
    fn test_build_merkle_tree_from_iterator() {
        //let data = [1, 2, 3, 4, 5];
        let left_data: Vec<Column> = vec![
            Column { rowid: "1".to_string(), name: "col1".to_string(), value: "col1_value".to_string() },
            Column { rowid: "1".to_string(), name: "col2".to_string(), value: "col2_value".to_string() },
            Column { rowid: "1".to_string(), name: "col3".to_string(), value: "col3_value".to_string() },
            Column { rowid: "1".to_string(), name: "col4".to_string(), value: "col4_value".to_string() },
            Column { rowid: "2".to_string(), name: "col1".to_string(), value: "col1_value".to_string() },
            Column { rowid: "2".to_string(), name: "col2".to_string(), value: "col2_value".to_string() },
            Column { rowid: "2".to_string(), name: "col3".to_string(), value: "col3_value".to_string() },
            Column { rowid: "2".to_string(), name: "col4".to_string(), value: "col4_value".to_string() },
        ];

        let right_data: Vec<Column> = vec![
            Column { rowid: "1".to_string(), name: "col1".to_string(), value: "colX1_value".to_string() },
            Column { rowid: "1".to_string(), name: "col2".to_string(), value: "col2_value".to_string() },
            Column { rowid: "1".to_string(), name: "col3".to_string(), value: "colX3_value".to_string() },
            Column { rowid: "1".to_string(), name: "col4".to_string(), value: "col4_value".to_string() },
            Column { rowid: "2".to_string(), name: "col1".to_string(), value: "col1_value".to_string() },
            Column { rowid: "2".to_string(), name: "col2".to_string(), value: "col2_value".to_string() },
            Column { rowid: "2".to_string(), name: "col3".to_string(), value: "colXX3_value".to_string() },
            Column { rowid: "2".to_string(), name: "col4".to_string(), value: "col4_value".to_string() },
        ];

        let store = HashMapStore::default();
        let hasher = MurmurHasher::default();
        let left_merkle = MerkleTree::from_coll(left_data, &store, &hasher).unwrap();
        let right_merkle = MerkleTree::from_coll(right_data, &store, &hasher).unwrap();

        let differences = Differ::get_differences(&store, &left_merkle, &right_merkle);
        println!("{:?}", &differences);

        //Differ::write_differences_to_file("diff.csv".to_string(), &differences);
        let expected_diff_vector = vec![
            Difference { row_id: "1".to_string(), col_name: "col1".to_string(), left_value: "col1_value".to_string(), right_value: "colX1_value".to_string() },
            Difference { row_id: "1".to_string(), col_name: "col3".to_string(), left_value: "col3_value".to_string(), right_value: "colX3_value".to_string() },
            Difference { row_id: "2".to_string(), col_name: "col3".to_string(), left_value: "col3_value".to_string(), right_value: "colXX3_value".to_string() }
        ];
        assert_eq!(&differences, &expected_diff_vector)
    }
}