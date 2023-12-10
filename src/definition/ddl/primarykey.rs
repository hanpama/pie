use crate::{
    definition::{
        ddl::coerce::{
            coerce_boolean_value, coerce_name, coerce_nodes, coerce_string_varargs_value,
        },
        base::Node,
        defaults,
    },
    snapshot::PrimaryKey,
};

use super::{context::Context, error::Error};

pub fn parse_primary_key_definition(
    schema_name: &str,
    table_name: &str,
    n: &Node,
) -> Result<PrimaryKey, Error> {
    assert_eq!(n.r#type, "constraint");

    let name = coerce_name(n)?;
    let child_node = coerce_nodes(n)?;
    let mut errors: Vec<Error> = Vec::new();

    let mut columns: Option<Vec<String>> = None;
    let mut deferrable: Option<bool> = None;
    let mut initially_deferred: Option<bool> = None;

    for cn in child_node {
        if let Err(e) = match cn.r#type {
            "primary key" => coerce_string_varargs_value(cn).and_then(|v| Ok(columns = Some(v))),
            "deferrable" => coerce_boolean_value(cn).and_then(|v| Ok(deferrable = Some(v))),
            "initially deferred" => {
                coerce_boolean_value(cn).and_then(|v| Ok(initially_deferred = Some(v)))
            }
            _ => Err(Error::new_unexpected_node(cn)),
        } {
            errors.push(e);
        }
    }

    if columns.is_none() {
        errors.push(Error::new_attribute_required(n, "primary key"));
    }
    if deferrable.is_none() {
        deferrable = Some(defaults::get_constraint_deferrable());
    }
    if initially_deferred.is_none() {
        initially_deferred = Some(defaults::get_constraint_initially_deferred());
    }

    if !errors.is_empty() {
        return Err(Error::new_has_errors(n, errors));
    }

    Ok(PrimaryKey {
        schema_name: schema_name.to_owned(),
        table_name: table_name.to_owned(),
        name: name,
        columns: columns.unwrap(),
        deferrable: deferrable.unwrap(),
        initially_deferred: initially_deferred.unwrap(),
    })
}

pub fn render_primary_key_definition(ctx: &Context, def: &PrimaryKey) -> Node {
    let mut subnodes: Vec<Node> = Vec::new();

    subnodes.push(Node::new("primary key").with_string_varargs_value(def.columns.clone()));

    if def.deferrable != defaults::get_constraint_deferrable() {
        subnodes.push(Node::new("deferrable").with_boolean_value(def.deferrable));
    }
    if def.initially_deferred != defaults::get_constraint_initially_deferred() {
        subnodes.push(Node::new("initially deferred").with_boolean_value(def.initially_deferred));
    }

    Node::new("constraint")
        .with_name(def.name.clone())
        .with_nodes(subnodes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_and_render_primary_key_constraint_definition() {
        let node = Node::new("constraint")
            .with_name("pk".to_owned())
            .with_nodes(vec![
                Node::new("primary key").with_string_value("id".to_owned()),
                Node::new("deferrable").with_boolean_value(true),
                Node::new("initially deferred").with_boolean_value(true),
            ]);
        let def = PrimaryKey {
            schema_name: "public".to_owned(),
            table_name: "user".to_owned(),
            name: "pk".to_owned(),
            columns: vec!["id".to_owned()],
            deferrable: true,
            initially_deferred: true,
        };
        let ctx = Context {
            schema_name: "public".to_owned(),
            table_name: Some("user".to_owned()),
        };
        let got_def = parse_primary_key_definition("public", "user", &node).unwrap();
        assert_eq!(def, got_def);
        let got_node = render_primary_key_definition(&ctx, &def);
        assert_eq!(node, got_node);
    }
}
