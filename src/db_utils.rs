use csv::StringRecord;

use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

/// Convert any database vale to String
pub struct AllString(String);

impl AllString {
    pub fn into(self) -> String {
        self.0
    }

    fn db_data_to_string(value: ValueRef) -> String {
        match value {
            ValueRef::Real(value) => value.to_string(),
            ValueRef::Text(value) => value.to_owned(),
            ValueRef::Integer(value) => value.to_string(),
            ValueRef::Null => "".to_owned(),
            _ => "Cannot parse binary".to_owned(),
        }
    }
}

impl FromSql for AllString {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        Ok(AllString(Self::db_data_to_string(value)))
    }
}

pub fn escape_columns(columns: &StringRecord) -> Vec<String> {
    columns.iter().map(|c| format!("\"{}\"", c)).collect()
}

pub fn escape_values(values: &StringRecord) -> Vec<String> {
    values
        .iter()
        .map(|c| c.replace("'", "''"))
        .map(|c| format!("'{}'", c))
        .collect()
}
