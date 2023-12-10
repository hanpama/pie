#[derive(Debug, PartialEq)]
pub struct Function {
    pub schema_name: String,
    pub name: String,

    pub body: String,
    pub language: String,
    pub returns: String,

    pub volatility: String,
}

impl Function {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
