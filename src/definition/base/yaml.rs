use serde_yaml;

use super::{
    super::base::{Node, NodeChild, Number, Value},
    error::Error,
};

static KEYWORDS: [&str; 33] = [
    "foreign key",
    "initially deferred",
    "not null",
    "on delete",
    "on update",
    "owned by",
    "primary key",
    "as",
    "cache",
    "check",
    "column",
    "constraint",
    "cycle",
    "default",
    "deferrable",
    "function",
    "volatility",
    "index",
    "language",
    "match",
    "maxvalue",
    "method",
    "minvalue",
    "on",
    "references",
    "returns",
    "schema",
    "sequence",
    "start",
    "table",
    "type",
    "unique",
    "view",
];

pub fn parse_mapping_to_nodes(val: &serde_yaml::Mapping) -> Result<Vec<Node>, Error> {
    let mut nodes = Vec::with_capacity(val.len());

    for (k, v) in val {
        let key = k.as_str().ok_or_else(|| Error {
            message: format!("Invalid key: {:?}", k),
        })?;

        let mut node = create_node_from_mapping_key(key)?;

        match v {
            serde_yaml::Value::Mapping(child_mapping) => {
                let nodes = parse_mapping_to_nodes(child_mapping)?;
                node.child = Some(NodeChild::Nodes(nodes));
            }
            serde_yaml::Value::Null => {
                node.child = None;
            }
            _ => {
                let value = parse_serde_value_to_ddl_value(v)?;
                node.child = Some(NodeChild::Value(value));
            }
        }

        nodes.push(node);
    }

    Ok(nodes)
}

fn create_node_from_mapping_key(key: &str) -> Result<Node, Error> {
    let mut node = Node::default();

    let mut found = false;
    for keyword in KEYWORDS.iter() {
        if key.to_lowercase().starts_with(keyword) {
            node.r#type = keyword;
            found = true;
            break;
        }
    }
    if !found {
        return Err(Error::new(format!("Invalid key: {}", key)));
    }
    let name = key[node.r#type.len()..].trim().to_string();
    if !name.is_empty() {
        node.name = Some(name);
    }
    Ok(node)
}

fn parse_serde_sequence_to_ddl_list(val: &serde_yaml::Sequence) -> Result<Vec<Value>, Error> {
    let mut result: Vec<Value> = Vec::with_capacity(val.len());
    for v in val {
        let value = parse_serde_value_to_ddl_value(v)?;
        result.push(value);
    }
    Ok(result)
}

fn parse_serde_value_to_ddl_value(val: &serde_yaml::Value) -> Result<Value, Error> {
    match val {
        serde_yaml::Value::String(val_string) => {
            return Ok(Value::String(val_string.to_string()));
        }
        serde_yaml::Value::Number(val_number) => {
            let parsed = parse_serde_number_to_ddl_number(val_number)?;
            return Ok(Value::Number(parsed));
        }
        serde_yaml::Value::Bool(val_bool) => {
            return Ok(Value::Boolean(*val_bool));
        }
        serde_yaml::Value::Mapping(_) => {
            unreachable!()
        }
        serde_yaml::Value::Sequence(val_sequence) => {
            let parsed = parse_serde_sequence_to_ddl_list(val_sequence)?;
            return Ok(Value::List(parsed));
        }
        _ => {
            return Err(Error::new(format!("Invalid value: {:?}", val)));
        }
    }
}

fn parse_serde_number_to_ddl_number(val: &serde_yaml::Number) -> Result<Number, Error> {
    if val.is_f64() {
        return Ok(Number::Float(val.as_f64().unwrap()));
    }
    if val.is_i64() {
        return Ok(Number::Integer(val.as_i64().unwrap()));
    }
    Err(Error::new(format!("Invalid number: {:?}", val)))
}

pub fn render_nodes_to_mapping(nodes: &Vec<Node>) -> serde_yaml::Value {
    let mut map = serde_yaml::Mapping::new();

    for node in nodes {
        let key = render_node_to_mapping_key(node);
        let value = render_node_to_mapping_value(node);
        map.insert(key, value);
    }

    serde_yaml::Value::Mapping(map)
}

fn render_node_to_mapping_key(node: &Node) -> serde_yaml::Value {
    let mut key = node.r#type.to_string();
    if let Some(name) = &node.name {
        key.push_str(" ");
        key.push_str(name);
    }
    serde_yaml::Value::String(key)
}

fn render_node_to_mapping_value(node: &Node) -> serde_yaml::Value {
    match &node.child {
        Some(value) => match value {
            NodeChild::Value(value) => {
                return render_value_to_serde_value(value);
            }
            NodeChild::Nodes(nodes) => {
                return render_nodes_to_mapping(nodes);
            }
        },
        None => serde_yaml::Value::Null,
    }
}

fn render_value_to_serde_value(value: &Value) -> serde_yaml::Value {
    match value {
        Value::Boolean(val) => {
            return serde_yaml::Value::Bool(*val);
        }
        Value::String(val) => {
            return serde_yaml::Value::String(val.to_string());
        }
        Value::Number(val) => {
            return serde_yaml::Value::Number(render_number_to_serde_number(val));
        }
        Value::List(val) => {
            return serde_yaml::Value::Sequence(render_list_to_serde_sequence(val));
        }
    }
}

fn render_number_to_serde_number(number: &Number) -> serde_yaml::Number {
    match number {
        Number::Float(val) => serde_yaml::Number::from(*val),
        Number::Integer(val) => serde_yaml::Number::from(*val),
    }
}

fn render_list_to_serde_sequence(list: &Vec<Value>) -> serde_yaml::Sequence {
    let mut sequence = serde_yaml::Sequence::new();
    for value in list {
        let val = render_value_to_serde_value(value);
        sequence.push(val);
    }
    sequence
}
