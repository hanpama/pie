#[derive(Debug, PartialEq)]
pub struct Sequence {
    pub schema_name: String,
    pub name: String,

    pub data_type: String,
    pub increment: i64,
    pub min_value: i64,
    pub max_value: i64,
    pub start: i64,
    pub cache: i64,
    pub cycle: bool,
    pub owned_by_table: Option<String>,
    pub owned_by_column: Option<String>,
}
