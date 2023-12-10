#[derive(PartialEq, Debug)]
pub struct Node {
    pub r#type: &'static str,
    pub name: Option<String>,
    pub child: Option<NodeChild>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            r#type: "",
            name: None,
            child: None,
        }
    }
}

impl Node {
    pub fn new(r#type: &'static str) -> Node {
        Node {
            r#type,
            name: None,
            child: None,
        }
    }
    pub fn get_name(&self) -> Option<String> {
        return self.name.clone();
    }
    pub fn get_nodes(&self) -> Option<&Vec<Node>> {
        return self.child.as_ref()?.to_nodes();
    }
    pub fn get_boolean_value(&self) -> Option<bool> {
        return self.child.as_ref()?.to_value()?.to_boolean();
    }
    pub fn get_number_integer_value(&self) -> Option<i64> {
        return self.child.as_ref()?.to_value()?.to_number_integer();
    }
    pub fn get_number_float_value(&self) -> Option<f64> {
        return self.child.as_ref()?.to_value()?.to_number_float();
    }
    pub fn get_string_value(&self) -> Option<String> {
        return self.child.as_ref()?.to_value()?.to_string();
    }
    pub fn get_string_varargs_value(&self) -> Option<Vec<String>> {
        return self.child.as_ref()?.to_value()?.to_string_varargs();
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        return self;
    }
    pub fn with_nodes(mut self, nodes: Vec<Node>) -> Self {
        self.child = Some(NodeChild::Nodes(nodes));
        return self;
    }
    pub fn with_boolean_value(mut self, b: bool) -> Self {
        self.child = Some(NodeChild::Value(Value::from_boolean(b)));
        return self;
    }
    pub fn with_number_integer_value(mut self, i: i64) -> Self {
        self.child = Some(NodeChild::Value(Value::from_number_integer(i)));
        return self;
    }
    pub fn with_number_float_value(mut self, f: f64) -> Self {
        self.child = Some(NodeChild::Value(Value::from_number_float(f)));
        return self;
    }
    pub fn with_string_value(mut self, s: String) -> Self {
        self.child = Some(NodeChild::Value(Value::from_string(s)));
        return self;
    }
    pub fn with_string_varargs_value(mut self, strings: Vec<String>) -> Self {
        self.child = Some(NodeChild::Value(Value::from_string_varargs(strings)));
        return self;
    }

    pub fn display_string(&self) -> String {
        let mut s = String::new();
        s.push_str(self.r#type);
        if let Some(name) = &self.name {
            s.push_str(" ");
            s.push_str(name);
        }
        s
    }
}

#[derive(PartialEq, Debug)]
pub enum NodeChild {
    Value(Value),
    Nodes(Vec<Node>),
}

impl NodeChild {
    fn to_nodes(&self) -> Option<&Vec<Node>> {
        match self {
            NodeChild::Nodes(nodes) => Some(nodes),
            _ => None,
        }
    }
    fn to_value(&self) -> Option<&Value> {
        match self {
            NodeChild::Value(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Value {
    Boolean(bool),
    String(String),
    Number(Number),
    List(Vec<Value>),
}

impl Value {
    pub fn to_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn to_number_integer(&self) -> Option<i64> {
        match self {
            Value::Number(n) => n.to_integer(),
            _ => None,
        }
    }
    pub fn to_number_float(&self) -> Option<f64> {
        match self {
            Value::Number(n) => n.to_float(),
            _ => None,
        }
    }
    pub fn to_string(&self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.to_string()),
            _ => None,
        }
    }
    pub fn to_string_varargs(&self) -> Option<Vec<String>> {
        match self {
            Value::List(values) => {
                let mut strings = Vec::new();
                for value in values {
                    strings.push(value.to_string()?);
                }
                return Some(strings);
            }
            Value::String(s) => {
                return Some(vec![s.to_string()]);
            }
            _ => None,
        }
    }

    pub fn from_boolean(b: bool) -> Value {
        Value::Boolean(b)
    }
    pub fn from_number_integer(i: i64) -> Value {
        Value::Number(Number::from_integer(i))
    }
    pub fn from_number_float(f: f64) -> Value {
        Value::Number(Number::from_float(f))
    }
    pub fn from_string(s: String) -> Value {
        Value::String(s)
    }
    pub fn from_string_varargs(strings: Vec<String>) -> Value {
        if strings.len() == 1 {
            return Value::from_string(strings[0].to_owned());
        }
        let mut values = Vec::new();
        for s in strings {
            values.push(Value::from_string(s));
        }
        Value::List(values)
    }
}

#[derive(PartialEq, Debug)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl Number {
    pub fn to_integer(&self) -> Option<i64> {
        match self {
            Number::Integer(i) => Some(*i),
            Number::Float(f) => None,
        }
    }
    pub fn to_float(&self) -> Option<f64> {
        match self {
            Number::Integer(i) => Some(*i as f64),
            Number::Float(f) => Some(*f),
        }
    }
    pub fn from_integer(i: i64) -> Number {
        Number::Integer(i)
    }
    pub fn from_float(f: f64) -> Number {
        Number::Float(f)
    }
}
