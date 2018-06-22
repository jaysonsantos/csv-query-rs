use std::iter::Map;

pub fn escape_columns(columns: Vec<String>) -> Vec<String> {
    columns.iter().map(|c| format!("\"{}\"", c)).collect()
}

pub fn escape_values(columns: Vec<String>) -> Vec<String> {
    columns
        .iter()
        .map(|c| c.replace("'", "''"))
        .map(|c| format!("'{}'", c))
        .collect()
}
