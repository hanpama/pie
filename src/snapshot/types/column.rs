#[derive(PartialEq, Debug)]
pub struct Column {
    pub schema_name: String,
    pub table_name: String,
    pub name: String,

    pub data_type: String,
    pub not_null: bool,
    pub default: Option<String>,
}

impl Column {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
