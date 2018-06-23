use csv::StringRecord;

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
