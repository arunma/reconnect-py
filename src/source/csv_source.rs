use anyhow::Result;

use crate::{Column, Record};
use crate::source::Source;

pub struct CSVSource {
    pub file_path: String,
}

impl Source for CSVSource {
    fn read(&self) -> Result<Vec<Record>> {
        //FIXME Sort by rowid
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_path(&self.file_path).unwrap();

        let headers = {
            reader.headers().unwrap().clone()
        };

        let mut return_records = Vec::new();
        let row_id_index = {
            let mut inn_rowid_index = None;
            for (index, hname) in headers.iter().enumerate() {
                if hname == "rowid" {
                    inn_rowid_index = Some(index);
                    break;
                }
            }
            inn_rowid_index.expect("Dataset does not contain a `rowid` column")
        };

        for rec_result in reader.records() {
            let csv_record = rec_result.expect("Not a CSV record");

            let mut col_data = Vec::new();
            for cindex in 0..csv_record.len() {
                let column = Column {
                    rowid: csv_record[row_id_index].to_string(),
                    name: headers[cindex].to_string(),
                    value: csv_record[cindex].to_string(),
                };
                col_data.push(column);
            }
            return_records.push(Record { rowid: headers[row_id_index].to_string(), data: col_data });
        }

        Ok(return_records)
    }
}


#[cfg(test)]
mod tests {
    use crate::source::csv_source::CSVSource;
    use crate::source::Source;

    #[test]
    fn test_construct_records() {
        let source = CSVSource {
            file_path: "./fixtures/test_demographics1.csv".to_string(),
            //file_path: "./fixtures/test_simple1.csv".to_string(),
        };
        let records = source.read().unwrap();
        assert_eq!(records.len(), 1000)
    }
}