#[derive(Debug, PartialEq)]
pub struct Index {
    pub schema_name: String,
    pub name: String,
    
    pub table_name: String,
    pub unique: bool,
    pub method: String,
    pub key_expressions: Vec<String>,
}
