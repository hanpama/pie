use crate::definition::base::{yaml::parse_mapping_to_nodes, Node, NodeChild, Value};

use super::yaml::render_nodes_to_mapping;

static YAML: &str = r#"view userview:
  column id: uuid
  column name: text
  column created at: timestamp with time zone
  as: SELECT id, name, "created at" from user
  column null_test: null
"#;

#[test]
fn test_parse_yaml_to_nodes() {
    let mapping: serde_yaml::Mapping = serde_yaml::from_str(YAML).unwrap();
    let got = parse_mapping_to_nodes(&mapping).unwrap();

    assert_eq!(
        got,
        vec![Node {
            r#type: "view",
            name: Some("userview".to_string()),
            child: Some(NodeChild::Nodes(vec![
                Node {
                    r#type: "column",
                    name: Some("id".to_string()),
                    child: Some(NodeChild::Value(Value::String("uuid".to_string()))),
                },
                Node {
                    r#type: "column",
                    name: Some("name".to_string()),
                    child: Some(NodeChild::Value(Value::String("text".to_string()))),
                },
                Node {
                    r#type: "column",
                    name: Some("created at".to_string()),
                    child: Some(NodeChild::Value(Value::String(
                        "timestamp with time zone".to_string()
                    ))),
                },
                Node {
                    r#type: "as",
                    name: None,
                    child: Some(NodeChild::Value(Value::String(
                        "SELECT id, name, \"created at\" from user".to_string()
                    ))),
                },
                Node {
                    r#type: "column",
                    name: Some("null_test".to_string()),
                    child: None,
                }
            ]))
        }]
    )
}

#[test]
fn test_render_nodes_to_yaml() {
    let nodes = vec![Node {
        r#type: "view",
        name: Some("userview".to_string()),
        child: Some(NodeChild::Nodes(vec![
            Node {
                r#type: "column",
                name: Some("id".to_string()),
                child: Some(NodeChild::Value(Value::String("uuid".to_string()))),
            },
            Node {
                r#type: "column",
                name: Some("name".to_string()),
                child: Some(NodeChild::Value(Value::String("text".to_string()))),
            },
            Node {
                r#type: "column",
                name: Some("created at".to_string()),
                child: Some(NodeChild::Value(Value::String(
                    "timestamp with time zone".to_string(),
                ))),
            },
            Node {
                r#type: "as",
                name: None,
                child: Some(NodeChild::Value(Value::String(
                    "SELECT id, name, \"created at\" from user".to_string(),
                ))),
            },
            Node {
                r#type: "column",
                name: Some("null_test".to_string()),
                child: None,
            },
        ])),
    }];

    let mapping = render_nodes_to_mapping(&nodes);
    let got = serde_yaml::to_string(&mapping).unwrap();

    assert_eq!(got, YAML);
}
