pub struct Context {
    pub schema_name: String,
    pub table_name: Option<String>,
}

impl Context {
    pub fn new(schema_name: String) -> Self {
        Self {
            schema_name,
            table_name: None,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            schema_name: "public".to_string(),
            table_name: None,
        }
    }
}
