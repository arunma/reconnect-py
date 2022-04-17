use crate::hash::MurmurHasher;
use crate::merkle::MerkleTree;
use crate::source::csv_source::CSVSource;
use crate::source::Source;
use crate::store::hashmap::HashMapStore;
use anyhow::Result;
use csv::Writer;
use crate::Column;
use crate::differ::{Differ, Difference};

pub struct CsvApi {}

pub struct CsvParams {
    pub left_csv_path: String,
    pub right_csv_path: String,
}

impl CsvApi {
    pub fn compare_csv(params: CsvParams) -> Result<Vec<Difference<>>> {
        let left_data = Self::get_data_from_csv_path(params.left_csv_path);
        let right_data = Self::get_data_from_csv_path(params.right_csv_path);

        let store = HashMapStore::default();
        let hasher = MurmurHasher::default();
        let left_merkle = MerkleTree::from_coll(left_data, &store, &hasher).unwrap();
        let right_merkle = MerkleTree::from_coll(right_data, &store, &hasher).unwrap();

        let differences = Differ::get_differences(&store, &left_merkle, &right_merkle);

        Ok(differences)
    }

    fn get_data_from_csv_path(csv_path: String) -> Vec<Column> {
        let source = CSVSource { file_path: csv_path };
        let records = source.read().unwrap();
        let mut data = Vec::new();
        for record in records.into_iter() {
            data.extend(record.data.into_iter())
        }
        data
    }


    pub fn write_differences_to_file(file_path: String, diffs: &Vec<Difference>) -> Result<()> {
        let mut writer = Writer::from_path(file_path)?;
        for diff in diffs {
            writer.serialize(diff)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::csv_api::{CsvApi, CsvParams};

    #[test]
    fn test_csv_compare() {
        /*let params = CsvParams {
            left_csv_path: "./fixtures/test_simple1.csv".to_string(),
            right_csv_path: "./fixtures/test_simple2.csv".to_string(),
        };*/
        let params = CsvParams {
            left_csv_path: "./fixtures/test_demographics1.csv".to_string(),
            right_csv_path: "./fixtures/test_demographics2.csv".to_string(),
        };
        let differences = CsvApi::compare_csv(params).unwrap();

        CsvApi::write_differences_to_file("diff.csv".to_string(), &differences).unwrap();
        println!("{:#?}", differences)

    }
}