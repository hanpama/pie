use super::super::base::Node;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedNode {
        node_display_string: String,
    },
    NameRequired {
        node_display_string: String,
    },
    ChildNodesRequired {
        node_display_string: String,
    },
    AttributeRequired {
        node_display_string: String,
        attribute: &'static str,
    },
    InvalidTypeAttribute {
        node_display_string: String,
        expected: &'static str,
    },
    InvalidTypeExpression {
        type_expression: String,
    },
    InvalidTableReference {
        table_reference: String,
    },
    InvalidColumnReference {
        column_reference: String,
    },
    HasErrors {
        node_display_string: String,
        errors: Vec<Error>,
    },
}

impl Error {
    pub fn new_unexpected_node(node: &Node) -> Error {
        Error::UnexpectedNode {
            node_display_string: node.display_string(),
        }
    }
    pub fn new_name_required(node: &Node) -> Error {
        Error::NameRequired {
            node_display_string: node.display_string(),
        }
    }
    pub fn new_child_nodes_required(node: &Node) -> Error {
        Error::ChildNodesRequired {
            node_display_string: node.display_string(),
        }
    }
    pub fn new_attribute_required(node: &Node, attribute: &'static str) -> Error {
        Error::AttributeRequired {
            node_display_string: node.display_string(),
            attribute,
        }
    }
    pub fn new_invalid_type_attribute(node: &Node, expected: &'static str) -> Error {
        Error::InvalidTypeAttribute {
            node_display_string: node.display_string(),
            expected,
        }
    }
    pub fn new_invalid_type_expression(type_expression: String) -> Error {
        Error::InvalidTypeExpression { type_expression }
    }
    pub fn new_invalid_table_reference(table_reference: String) -> Error {
        Error::InvalidTableReference { table_reference }
    }
    pub fn new_invalid_column_reference(column_reference: String) -> Error {
        Error::InvalidColumnReference { column_reference }
    }
    pub fn new_has_errors(node: &Node, errors: Vec<Error>) -> Error {
        Error::HasErrors {
            node_display_string: node.display_string(),
            errors,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnexpectedNode {
                node_display_string,
            } => {
                write!(f, "unexpected node: {}", node_display_string)
            }
            Error::NameRequired {
                node_display_string,
            } => {
                write!(f, "name required: {}", node_display_string)
            }
            Error::ChildNodesRequired {
                node_display_string,
            } => {
                write!(f, "child nodes required: {}", node_display_string)
            }
            Error::AttributeRequired {
                node_display_string,
                attribute,
            } => {
                write!(
                    f,
                    "attribute required: {} {}",
                    node_display_string, attribute
                )
            }
            Error::InvalidTypeAttribute {
                node_display_string,
                expected,
            } => {
                write!(
                    f,
                    "invalid type attribute: {} expected {}",
                    node_display_string, expected
                )
            }
            Error::InvalidTypeExpression { type_expression } => {
                write!(f, "invalid type expression: {}", type_expression)
            }
            Error::InvalidTableReference { table_reference } => {
                write!(f, "invalid table reference: {}", table_reference)
            }
            Error::InvalidColumnReference { column_reference } => {
                write!(f, "invalid column reference: {}", column_reference)
            }
            Error::HasErrors {
                node_display_string,
                errors,
            } => {
                write!(f, "has errors: {}", node_display_string)?;
                for error in errors {
                    write!(f, "\n  {}", error)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for Error {}
