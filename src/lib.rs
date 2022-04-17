extern crate serde;
extern crate serde_derive;
extern crate csv;

mod merkle;
mod store;
mod hash;
mod perf_prop;

pub mod differ;
pub mod source;
pub mod csv_api;

pub trait Data:Into<Vec<u8>>{}

#[derive(Debug, PartialEq, Clone)]
pub enum DValue {
    // FIXME: Add more data types
    Integer(i32),
    Long(i64),
    String(String),
    Float(f32),
    Double(f64),
}

impl Data for DValue{}

#[derive(Debug, PartialEq)]
pub struct Column {
    rowid: String,
    name: String,
    value: String,
}

impl Data for Column{}

#[derive(Debug, PartialEq)]
pub struct Record {
    rowid: String,
    data: Vec<Column>,
}

// #[derive(Debug, PartialEq, Clone)]
// struct DColumn {
//     name: String,
//     value: String,
static COLON_BYTE: u8 = b':';
static COLON_STR: &str = ":";

impl From<DValue> for Vec<u8> {
    fn from(data: DValue) -> Self {
        match data {
            DValue::Integer(int) => int.to_le_bytes().to_vec(),
            DValue::Long(lng) => lng.to_le_bytes().to_vec(),
            DValue::String(str) => str.as_bytes().to_vec(),
            DValue::Float(flt) => flt.to_le_bytes().to_vec(),
            DValue::Double(flt) => flt.to_le_bytes().to_vec(),
        }
    }
}


impl From<Column> for Vec<u8> {
    fn from(column: Column) -> Self {
        let Column { rowid, name, value } = column;
        let mut byte_vec: Vec<u8> = Vec::new();
        byte_vec.extend(rowid.as_bytes());
        byte_vec.push(COLON_BYTE);
        byte_vec.extend(name.as_bytes());
        byte_vec.push(COLON_BYTE);
        byte_vec.extend(value.as_bytes());
        byte_vec
    }
}
