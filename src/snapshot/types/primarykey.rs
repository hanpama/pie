#[derive(PartialEq, Debug)]
pub struct PrimaryKey {
    pub schema_name: String,
    pub table_name: String,
    pub name: String,

    pub columns: Vec<String>,

    pub deferrable: bool,
    pub initially_deferred: bool,
}
