use sqlite::{Type, Value};

pub fn db_data_to_string(data: &Value) -> String {
    match data.kind() {
        Type::Float => data.as_float().unwrap().to_string(),
        Type::String => data.as_string().unwrap().to_owned(),
        Type::Integer => data.as_integer().unwrap().to_string(),
        Type::Null => "".to_owned(),
        _ => "Cannot parse binary".to_owned(),
    }
}

pub fn db_data_to_csv_output(data: &Value) -> String {
    format!("\"{}\"", db_data_to_string(data))
}
