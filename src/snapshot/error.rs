#[derive(Debug, PartialEq)]
pub enum SnapshotError {
    ObjectNotFound {
        path: Vec<String>,
        r#type: &'static str,
    },
    ObjectAlreadyExists {
        path: Vec<String>,
        r#type: &'static str,
    },
    ObjectHasUnexpectedType {
        path: Vec<String>,
        expected: &'static str,
        actual: &'static str,
    },
}

impl SnapshotError {
    fn object_not_found(path: Vec<String>, r#type: &'static str) -> Self {
        Self::ObjectNotFound { path, r#type }
    }
    fn object_already_exists(path: Vec<String>, r#type: &'static str) -> Self {
        Self::ObjectAlreadyExists { path, r#type }
    }

    pub fn schema_not_found(schema: &str) -> Self {
        Self::object_not_found(vec![schema.to_string()], "schema")
    }
    pub fn schema_already_exists(schema: &str) -> Self {
        Self::object_already_exists(vec![schema.to_string()], "schema")
    }

    pub fn relation_not_found(schema: &str, relation: &str) -> Self {
        Self::object_not_found(vec![schema.to_string(), relation.to_string()], "relation")
    }
    pub fn relation_already_exists(schema: &str, relation: &str) -> Self {
        Self::object_already_exists(vec![schema.to_string(), relation.to_string()], "relation")
    }

    pub fn function_not_found(schema: &str, function: &str) -> Self {
        Self::object_not_found(vec![schema.to_string(), function.to_string()], "function")
    }
    pub fn function_already_exists(schema: &str, function: &str) -> Self {
        Self::object_already_exists(vec![schema.to_string(), function.to_string()], "function")
    }

    pub fn relation_has_unexpected_type(
        schema: &str,
        relation: &str,
        expected: &'static str,
        actual: &'static str,
    ) -> Self {
        Self::ObjectHasUnexpectedType {
            path: vec![schema.to_string(), relation.to_string()],
            expected,
            actual,
        }
    }

    pub fn column_not_found(schema: &str, relation: &str, column: &str) -> Self {
        let path = vec![schema.to_string(), relation.to_string(), column.to_string()];
        Self::object_not_found(path, "column")
    }
    pub fn column_already_exists(schema: &str, relation: &str, column: &str) -> Self {
        let path = vec![schema.to_string(), relation.to_string(), column.to_string()];
        Self::object_already_exists(path, "column")
    }

    pub fn constraint_not_found(schema: &str, relation: &str, constraint: &str) -> Self {
        let path = vec![
            schema.to_string(),
            relation.to_string(),
            constraint.to_string(),
        ];
        Self::object_not_found(path, "constraint")
    }
    pub fn constraint_already_exists(schema: &str, relation: &str, constraint: &str) -> Self {
        let path = vec![
            schema.to_string(),
            relation.to_string(),
            constraint.to_string(),
        ];
        Self::object_already_exists(path, "constraint")
    }

    pub fn constraint_has_unexpected_type(
        schema: &str,
        table: &str,
        constraint: &str,
        expected: &'static str,
        actual: &'static str,
    ) -> Self {
        Self::ObjectHasUnexpectedType {
            path: vec![
                schema.to_string(),
                table.to_string(),
                constraint.to_string(),
            ],
            expected: expected,
            actual: actual,
        }
    }
}

impl std::fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        panic!("unimplemented")
    }
}

impl std::error::Error for SnapshotError {}
