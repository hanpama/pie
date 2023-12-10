#[derive(Debug, PartialEq)]
pub struct Check {
    pub schema_name: String,
    pub table_name: String,
    pub name: String,
    pub expression: String,
    pub deferrable: bool,
    pub initially_deferred: bool,
}
