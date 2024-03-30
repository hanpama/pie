use crate::{
    definition::{
        base::Node,
        ddl::coerce::{coerce_name, coerce_nodes, coerce_string_value},
        defaults,
    },
    snapshot::Function,
};

use super::{context::Context, error::Error};

pub fn parse_function_definition(ctx: &Context, n: &Node) -> Result<Function, Error> {
    assert_eq!(n.r#type, "function");

    let name = coerce_name(n)?;
    let child_nodes = coerce_nodes(n)?;
    let mut errors: Vec<Error> = Vec::new();

    let mut body: Option<String> = None;
    let mut language: Option<String> = None;
    let mut returns: Option<String> = None;
    let mut volatility: Option<String> = None;

    for cn in child_nodes {
        if let Err(e) = match cn.r#type {
            "as" => coerce_string_value(cn).and_then(|v| Ok(body = Some(v))),
            "language" => coerce_string_value(cn).and_then(|v| Ok(language = Some(v))),
            "returns" => coerce_string_value(cn).and_then(|v| Ok(returns = Some(v))),
            "volatility" => coerce_string_value(cn).and_then(|v| Ok(volatility = Some(v))),
            _ => Err(Error::new_unexpected_node(cn)),
        } {
            errors.push(e);
        }
    }

    if body.is_none() {
        errors.push(Error::new_attribute_required(n, "body"));
    }
    if language.is_none() {
        language = Some(defaults::get_function_language());
    }
    if returns.is_none() {
        returns = Some(defaults::get_function_returns());
    }
    if volatility.is_none() {
        volatility = Some(defaults::get_function_volatility());
    }

    if !errors.is_empty() {
        return Err(Error::new_has_errors(n, errors));
    }

    Ok(Function {
        schema_name: ctx.schema_name.clone(),
        name: name,
        body: body.unwrap(),
        language: language.unwrap(),
        returns: returns.unwrap(),
        volatility: volatility.unwrap(),
    })
}

pub fn render_function_definition(_ctx: &Context, def: &Function) -> Node {
    let mut subnodes: Vec<Node> = Vec::new();

    subnodes.push(Node::new("as").with_string_value(def.body.clone()));

    if def.language != defaults::get_function_language() {
        subnodes.push(Node::new("language").with_string_value(def.language.clone()));
    }
    if def.returns != defaults::get_function_returns() {
        subnodes.push(Node::new("returns").with_string_value(def.returns.clone()));
    }
    if def.volatility != defaults::get_function_volatility() {
        subnodes.push(Node::new("volatility").with_string_value(def.volatility.clone()));
    }

    Node::new("function")
        .with_name(def.name.clone())
        .with_nodes(subnodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_render_function_definition() {
        let node = Node::new("function")
            .with_name("myfunc()".to_owned())
            .with_nodes(vec![
                Node::new("as").with_string_value("RETURN 3;".to_owned()),
                Node::new("language").with_string_value("plpgsql".to_owned()),
                Node::new("returns").with_string_value("integer".to_owned()),
                Node::new("volatility").with_string_value("immutable".to_owned()),
            ]);
        let def = Function {
            schema_name: "public".to_owned(),
            name: "myfunc()".to_owned(),
            body: "RETURN 3;".to_owned(),
            language: "plpgsql".to_owned(),
            returns: "integer".to_owned(),
            volatility: "immutable".to_owned(),
        };
        let ctx = Context {
            schema_name: "public".to_owned(),
            table_name: None,
        };
        let got_def = parse_function_definition(&ctx, &node).unwrap();
        assert_eq!(got_def, def);
        let got_node = render_function_definition(&ctx, &def);
        assert_eq!(got_node, node);
    }
}
