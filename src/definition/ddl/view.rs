use crate::{
    definition::{
        ddl::coerce::{coerce_name, coerce_nodes, coerce_string_value},
        base::Node,
    },
    snapshot::View,
};

use super::{context::Context, error::Error};

pub fn parse_view_definition(ctx: &Context, n: &Node) -> Result<View, Error> {
    assert_eq!(n.r#type, "view");

    let name = coerce_name(n)?;
    let child_nodes = coerce_nodes(n)?;
    let mut errors: Vec<Error> = Vec::new();

    let mut query: Option<String> = None;

    for cn in child_nodes {
        if let Err(e) = match cn.r#type {
            "as" => coerce_string_value(cn).and_then(|v| Ok(query = Some(v))),
            _ => Err(Error::new_unexpected_node(cn)),
        } {
            errors.push(e);
        }
    }

    if query.is_none() {
        errors.push(Error::new_attribute_required(n, "as"));
    }

    if !errors.is_empty() {
        return Err(Error::new_has_errors(n, errors));
    }

    Ok(View {
        schema_name: ctx.schema_name.clone(),
        name,
        query: query.unwrap(),
    })
}

pub fn render_view_definition(ctx: &Context, def: &View) -> Node {
    assert!(def.schema_name == ctx.schema_name);

    let mut node = Node::default();
    node.r#type = "view";
    node.name = Some(def.name.clone());

    let mut subnodes: Vec<Node> = Vec::new();

    if !def.query.is_empty() {
        subnodes.push(Node::new("as").with_string_value(def.query.to_owned()));
    }
    return node.with_nodes(subnodes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_render_view_definition() {
        let node = Node::new("view")
            .with_name("userview".to_owned())
            .with_nodes(vec![
                Node::new("as").with_string_value("select * from users".to_owned())
            ]);
        let def = View {
            schema_name: "public".to_owned(),
            name: "userview".to_owned(),
            query: "select * from users".to_owned(),
        };
        let ctx = Context {
            schema_name: "public".to_owned(),
            table_name: None,
        };
        let got_def = parse_view_definition(&ctx, &node).unwrap();
        assert_eq!(got_def, def);
        let got_node = render_view_definition(&ctx, &def);
        assert_eq!(got_node, node);
    }
}
