use crate::{
    definition::{
        base::Node,
        ddl::coerce::{
            coerce_boolean_value, coerce_name, coerce_name_and_string_varargs_value, coerce_nodes,
            coerce_string_value, coerce_string_varargs_value,
        },
        defaults,
    },
    snapshot::ForeignKey,
};

use super::{context::Context, error::Error};

pub fn parse_foreign_key_constraint_definition(
    schema_name: &str,
    table_name: &str,
    n: &Node,
) -> Result<ForeignKey, Error> {
    assert_eq!(n.r#type, "constraint");

    let name: String = coerce_name(n)?;
    let child_nodes: &Vec<Node> = coerce_nodes(n)?;

    let mut columns: Option<Vec<String>> = None;
    let mut target_schema: Option<String> = None;
    let mut target_table: Option<String> = None;
    let mut target_columns: Vec<String> = Vec::new();
    let mut deferrable: Option<bool> = None;
    let mut initially_deferred: Option<bool> = None;
    let mut match_option: Option<String> = None;
    let mut update_rule: Option<String> = None;
    let mut delete_rule: Option<String> = None;

    let mut errors: Vec<Error> = Vec::new();

    for cn in child_nodes {
        if let Err(e) = match cn.r#type {
            "foreign key" => coerce_string_varargs_value(cn).and_then(|v| Ok(columns = Some(v))),
            "references" => coerce_name_and_string_varargs_value(cn).and_then(|(name, v)| {
                let reference = parse_table_reference(schema_name, &name)?;
                target_schema = Some(reference.0);
                target_table = Some(reference.1);
                target_columns = v;
                Ok(())
            }),
            "deferrable" => coerce_boolean_value(cn).and_then(|v| Ok(deferrable = Some(v))),
            "initially deferred" => {
                coerce_boolean_value(cn).and_then(|v| Ok(initially_deferred = Some(v)))
            }
            "match" => coerce_string_value(cn).and_then(|v| Ok(match_option = Some(v))),
            "on update" => coerce_string_value(cn).and_then(|v| Ok(update_rule = Some(v))),
            "on delete" => coerce_string_value(cn).and_then(|v| Ok(delete_rule = Some(v))),
            _ => Err(Error::new_unexpected_node(cn)),
        } {
            errors.push(e);
        }
    }
    if columns.is_none() {
        errors.push(Error::new_attribute_required(n, "columns"));
    }
    if target_schema.is_none() || target_table.is_none() || target_columns.is_empty() {
        errors.push(Error::new_attribute_required(n, "references"));
    }
    if deferrable.is_none() {
        deferrable = Some(defaults::get_constraint_deferrable());
    }
    if initially_deferred.is_none() {
        initially_deferred = Some(defaults::get_constraint_initially_deferred());
    }
    if match_option.is_none() {
        match_option = Some(defaults::get_foreign_key_match_option());
    }
    if update_rule.is_none() {
        update_rule = Some(defaults::get_foreign_key_update_rule());
    }
    if delete_rule.is_none() {
        delete_rule = Some(defaults::get_foreign_key_delete_rule());
    }

    if !errors.is_empty() {
        return Err(Error::new_has_errors(n, errors));
    }

    return Ok(ForeignKey {
        schema_name: schema_name.to_owned(),
        table_name: table_name.to_owned(),
        name: name,
        columns: columns.unwrap(),
        target_schema: target_schema.unwrap(),
        target_table: target_table.unwrap(),
        target_columns: target_columns,
        deferrable: deferrable.unwrap(),
        initially_deferred: initially_deferred.unwrap(),
        match_option: match_option.unwrap(),
        update_rule: update_rule.unwrap(),
        delete_rule: delete_rule.unwrap(),
    });
}

fn parse_table_reference(search_path: &str, input: &str) -> Result<(String, String), Error> {
    let parts: Vec<&str> = input.split('.').collect();
    match parts.len() {
        1 => Ok((search_path.to_owned(), parts[0].to_owned())),
        2 => Ok((parts[0].to_owned(), parts[1].to_owned())),
        _ => Err(Error::new_invalid_table_reference(input.to_owned())),
    }
}

fn render_table_reference(ctx: &Context, schema_name: &str, table_name: &str) -> String {
    if schema_name == ctx.schema_name {
        table_name.to_owned()
    } else {
        format!("{}.{}", schema_name, table_name)
    }
}

pub fn render_foreign_key_constraint_definition(ctx: &Context, def: &ForeignKey) -> Node {
    let mut subnodes: Vec<Node> = Vec::new();

    subnodes.push(Node::new("foreign key").with_string_varargs_value(def.columns.clone()));
    subnodes.push(
        Node::new("references")
            .with_name(render_table_reference(
                ctx,
                &def.target_schema,
                &def.target_table,
            ))
            .with_string_varargs_value(def.target_columns.clone()),
    );

    if def.deferrable != defaults::get_constraint_deferrable() {
        subnodes.push(Node::new("deferrable").with_boolean_value(def.deferrable));
    }
    if def.initially_deferred != defaults::get_constraint_initially_deferred() {
        subnodes.push(Node::new("initially deferred").with_boolean_value(def.initially_deferred));
    }
    if def.match_option != defaults::get_foreign_key_match_option() {
        subnodes.push(Node::new("match").with_string_value(def.match_option.clone()));
    }
    if def.update_rule != defaults::get_foreign_key_update_rule() {
        subnodes.push(Node::new("on update").with_string_value(def.update_rule.clone()));
    }
    if def.delete_rule != defaults::get_foreign_key_delete_rule() {
        subnodes.push(Node::new("on delete").with_string_value(def.delete_rule.clone()));
    }

    return Node::new("constraint")
        .with_name(def.name.clone())
        .with_nodes(subnodes);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_and_render_foreign_key_constraint_definition() {
        let node = Node::new("constraint")
            .with_name("test".to_owned())
            .with_nodes(vec![
                Node::new("foreign key")
                    .with_string_varargs_value(vec!["a".to_owned(), "b".to_owned()]),
                Node::new("references")
                    .with_name("test2".to_owned())
                    .with_string_varargs_value(vec!["c".to_owned(), "d".to_owned()]),
                Node::new("deferrable").with_boolean_value(true),
                Node::new("initially deferred").with_boolean_value(true),
                Node::new("match").with_string_value("full".to_owned()),
                Node::new("on update").with_string_value("cascade".to_owned()),
                Node::new("on delete").with_string_value("cascade".to_owned()),
            ]);
        let def = ForeignKey {
            schema_name: "public".to_owned(),
            table_name: "test".to_owned(),
            name: "test".to_owned(),
            columns: vec!["a".to_owned(), "b".to_owned()],
            target_schema: "public".to_owned(),
            target_table: "test2".to_owned(),
            target_columns: vec!["c".to_owned(), "d".to_owned()],
            deferrable: true,
            initially_deferred: true,
            match_option: "full".to_owned(),
            update_rule: "cascade".to_owned(),
            delete_rule: "cascade".to_owned(),
        };
        let ctx = Context {
            schema_name: "public".to_owned(),
            table_name: Some("test".to_owned()),
        };
        let got_def = parse_foreign_key_constraint_definition("public", "test", &node).unwrap();
        assert_eq!(got_def, def);
        let got_node = render_foreign_key_constraint_definition(&ctx, &def);
        assert_eq!(got_node, node);
    }
}
