#[derive(Debug, PartialEq)]
pub struct ForeignKey {
    pub schema_name: String,
    pub table_name: String,
    pub name: String,

    pub columns: Vec<String>,
    pub target_schema: String,
    pub target_table: String,
    pub target_columns: Vec<String>,
    pub match_option: String,
    pub update_rule: String,
    pub delete_rule: String,

    pub deferrable: bool,
    pub initially_deferred: bool,
}
