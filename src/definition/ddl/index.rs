use crate::{
    definition::{
        ddl::coerce::{
            coerce_boolean_value, coerce_name, coerce_name_and_string_varargs_value, coerce_nodes,
            coerce_string_value, coerce_string_varargs_value,
        },
        base::Node,
        defaults,
    },
    snapshot::Index,
};

use super::{context::Context, error::Error};

pub fn parse_index_definition(ctx: &Context, n: &Node) -> Result<Index, Error> {
    assert_eq!(n.r#type, "index");

    let name = coerce_name(n)?;
    let child_nodes = coerce_nodes(n)?;
    let mut errors: Vec<Error> = Vec::new();

    let mut table_name: Option<String> = None;
    let mut unique: Option<bool> = None;
    let mut method: Option<String> = None;
    let mut key_expressions: Option<Vec<String>> = None;

    for cn in child_nodes {
        if let Err(e) = match cn.r#type {
            "unique" => coerce_boolean_value(cn).and_then(|v| Ok(unique = Some(v))),
            "on" => coerce_name_and_string_varargs_value(cn).and_then(|(name, v)| {
                table_name = Some(name);
                key_expressions = Some(v);
                Ok(())
            }),
            "using" => coerce_string_value(cn).and_then(|v| Ok(method = Some(v))),
            _ => Err(Error::new_unexpected_node(cn)),
        } {
            errors.push(e);
        }
    }

    if unique.is_none() {
        unique = Some(defaults::get_index_unique());
    }
    if method.is_none() {
        method = Some(defaults::get_default_index_method());
    }
    if table_name.is_none() || key_expressions.is_none() {
        errors.push(Error::new_attribute_required(n, "on"));
    }

    Ok(Index {
        schema_name: ctx.schema_name.clone(),
        table_name: table_name.unwrap(),
        name: name,
        unique: unique.unwrap(),
        method: method.unwrap(),
        key_expressions: key_expressions.unwrap(),
    })
}

pub fn render_index_definition(ctx: &Context, def: &Index) -> Node {
    let mut subnodes: Vec<Node> = Vec::new();

    if def.unique != defaults::get_index_unique() {
        subnodes.push(Node::new("unique").with_boolean_value(def.unique));
    }
    if def.method != defaults::get_default_index_method() {
        subnodes.push(Node::new("using").with_string_value(def.method.clone()));
    }
    subnodes.push(
        Node::new("on")
            .with_name(def.table_name.clone())
            .with_string_varargs_value(def.key_expressions.clone()),
    );

    Node::new("index")
        .with_name(def.name.clone())
        .with_nodes(subnodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_render_index_definition() {
        let node = Node::new("index")
            .with_name("user_id_idx".to_owned())
            .with_nodes(vec![Node::new("on")
                .with_name("users".to_owned())
                .with_string_varargs_value(vec![
                    "user_id".to_owned(),
                    "username".to_owned(),
                ])]);
        let def = Index {
            schema_name: "public".to_owned(),
            table_name: "users".to_owned(),
            name: "user_id_idx".to_owned(),
            unique: defaults::get_index_unique(),
            method: defaults::get_default_index_method(),
            key_expressions: vec!["user_id".to_owned(), "username".to_owned()],
        };
        let ctx = Context {
            schema_name: "public".to_owned(),
            table_name: None,
        };
        let got_def = parse_index_definition(&ctx, &node).unwrap();
        assert_eq!(got_def, def);

        let got_node = render_index_definition(&ctx, &def);
        assert_eq!(got_node, node);
    }
}
