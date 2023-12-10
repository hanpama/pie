use super::super::base::Node;
use super::error::Error;

pub fn coerce_name(n: &Node) -> Result<String, Error> {
    n.get_name().ok_or_else(|| Error::new_name_required(n))
}

pub fn coerce_nodes(n: &Node) -> Result<&Vec<Node>, Error> {
    n.get_nodes()
        .ok_or_else(|| Error::new_child_nodes_required(n))
}

pub fn coerce_string_value(n: &Node) -> Result<String, Error> {
    n.get_string_value()
        .ok_or_else(|| Error::new_invalid_type_attribute(n, "string"))
}

pub fn coerce_name_and_string_value(n: &Node) -> Result<(String, String), Error> {
    let name = n.get_name();
    let string = n.get_string_value();

    if name.is_none() {
        return Err(Error::new_name_required(n));
    }
    if string.is_none() {
        return Err(Error::new_invalid_type_attribute(n, "string"));
    }
    return Ok((name.unwrap(), string.unwrap()));
}

pub fn coerce_name_and_string_varargs_value(n: &Node) -> Result<(String, Vec<String>), Error> {
    let name = n.get_name();
    let string_varargs = n.get_string_varargs_value();

    if name.is_none() {
        return Err(Error::new_name_required(n));
    }
    if string_varargs.is_none() {
        return Err(Error::new_invalid_type_attribute(n, "...string"));
    }
    return Ok((name.unwrap(), string_varargs.unwrap()));
}

pub fn coerce_boolean_value(n: &Node) -> Result<bool, Error> {
    n.get_boolean_value()
        .ok_or_else(|| Error::new_invalid_type_attribute(n, "boolean"))
}

pub fn coerce_integer_value(n: &Node) -> Result<i64, Error> {
    n.get_number_integer_value()
        .ok_or_else(|| Error::new_invalid_type_attribute(n, "integer"))
}

pub fn coerce_float_value(n: &Node) -> Result<f64, Error> {
    n.get_number_float_value()
        .ok_or_else(|| Error::new_invalid_type_attribute(n, "float"))
}

pub fn coerce_string_varargs_value(n: &Node) -> Result<Vec<String>, Error> {
    n.get_string_varargs_value()
        .ok_or_else(|| Error::new_invalid_type_attribute(n, "...string"))
}
