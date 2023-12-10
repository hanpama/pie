use crate::{
    definition::{
        base::Node,
        ddl::coerce::{coerce_boolean_value, coerce_name, coerce_nodes, coerce_string_value},
        defaults,
    },
    snapshot::Check,
};

use super::{context::Context, error::Error};

pub fn parse_check_constraint_definition(
    schema_name: &str,
    table_name: &str,
    n: &Node,
) -> Result<Check, Error> {
    assert_eq!(n.r#type, "constraint");

    let name: String = coerce_name(n)?;
    let child_nodes = coerce_nodes(n)?;
    let mut errors: Vec<Error> = Vec::new();

    let mut expression: Option<String> = None;
    let mut deferrable: Option<bool> = None;
    let mut initially_deferred: Option<bool> = None;

    for cn in child_nodes {
        if let Err(e) = match cn.r#type {
            "check" => {
                coerce_string_value(cn).and_then(|v| Ok(expression = Some(v))) //
            }
            "deferrable" => {
                coerce_boolean_value(cn).and_then(|v| Ok(deferrable = Some(v))) //
            }
            "initially deferred" => {
                coerce_boolean_value(cn).and_then(|v| Ok(initially_deferred = Some(v)))
            }
            _ => Err(Error::new_unexpected_node(cn)),
        } {
            errors.push(e);
        }
    }

    if expression.is_none() {
        errors.push(Error::new_attribute_required(n, "expression"));
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

    Ok(Check {
        schema_name: schema_name.to_owned(),
        table_name: table_name.to_owned(),
        name: name,
        expression: expression.unwrap(),
        deferrable: deferrable.unwrap(),
        initially_deferred: initially_deferred.unwrap(),
    })
}

pub fn render_check_constraint_definition(ctx: &Context, def: &Check) -> Node {
    let mut subnodes: Vec<Node> = Vec::new();

    subnodes.push(Node::new("check").with_string_value(def.expression.clone()));

    if def.deferrable != defaults::get_constraint_deferrable() {
        let subnode = Node::new("deferrable").with_boolean_value(def.deferrable);
        subnodes.push(subnode);
    }
    if def.initially_deferred != defaults::get_constraint_initially_deferred() {
        let subnode = Node::new("initially deferred").with_boolean_value(def.initially_deferred);
        subnodes.push(subnode);
    }

    return Node::new("constraint")
        .with_name(def.name.clone())
        .with_nodes(subnodes);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_and_render_check_constraint_definition() {
        use super::parse_check_constraint_definition;

        let node = Node::new("constraint")
            .with_name("test".to_owned())
            .with_nodes(vec![
                Node::new("check").with_string_value("a > 0".to_owned()),
                Node::new("deferrable").with_boolean_value(true),
            ]);
        let def = Check {
            schema_name: "public".to_owned(),
            table_name: "user".to_owned(),
            name: "test".to_owned(),
            expression: "a > 0".to_owned(),
            deferrable: true,
            initially_deferred: false,
        };
        let ctx = Context {
            schema_name: "public".to_owned(),
            table_name: Some("user".to_owned()),
        };
        let got_def = parse_check_constraint_definition("public", "user", &node).unwrap();
        assert_eq!(got_def, def);
        let got_node = render_check_constraint_definition(&ctx, &def);
        assert_eq!(got_node, node);
    }
}
