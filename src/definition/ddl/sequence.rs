use crate::{
    definition::{
        ddl::coerce::{
            coerce_boolean_value, coerce_integer_value, coerce_name, coerce_name_and_string_value,
            coerce_nodes, coerce_string_value,
        },
        base::Node,
        defaults,
    },
    snapshot::Sequence,
};

use super::{context::Context, error::Error};

pub fn parse_sequence_definition(ctx: &Context, n: &Node) -> Result<Sequence, Error> {
    assert_eq!(n.r#type, "sequence");

    let name = coerce_name(n)?;
    let child_nodes = coerce_nodes(n)?;
    let mut errors: Vec<Error> = Vec::new();

    let mut data_type: Option<String> = None;
    let mut increment: Option<i64> = None;
    let mut min_value: Option<i64> = None;
    let mut max_value: Option<i64> = None;
    let mut start: Option<i64> = None;
    let mut cache: Option<i64> = None;
    let mut cycle: Option<bool> = None;
    let mut owned_by_table: Option<String> = None;
    let mut owned_by_column: Option<String> = None;

    for d in child_nodes {
        if let Err(e) = match d.r#type {
            "as" => coerce_string_value(d).and_then(|v| Ok(data_type = Some(v))),
            "increment" => coerce_integer_value(d).and_then(|v| Ok(increment = Some(v))),
            "minvalue" => coerce_integer_value(d).and_then(|v| Ok(min_value = Some(v))),
            "maxvalue" => coerce_integer_value(d).and_then(|v| Ok(max_value = Some(v))),
            "start" => coerce_integer_value(d).and_then(|v| Ok(start = Some(v))),
            "cache" => coerce_integer_value(d).and_then(|v| Ok(cache = Some(v))),
            "cycle" => coerce_boolean_value(d).and_then(|v| Ok(cycle = Some(v))),
            "owned by" => coerce_name_and_string_value(d).and_then(|(name, v)| {
                owned_by_table = Some(name);
                owned_by_column = Some(v);
                Ok(())
            }),
            _ => Err(Error::new_unexpected_node(d)),
        } {
            errors.push(e);
        }
    }

    if data_type.is_none() {
        data_type = Some(defaults::get_sequence_data_type());
    }
    if increment.is_none() {
        increment = Some(defaults::get_sequence_increment());
    }
    if min_value.is_none() {
        min_value = Some(defaults::get_sequence_min_value(
            increment.unwrap(),
            data_type.as_ref().unwrap(),
        ));
    }
    if max_value.is_none() {
        max_value = Some(defaults::get_sequence_max_value(
            increment.unwrap(),
            data_type.as_ref().unwrap(),
        ));
    }
    if start.is_none() {
        start = Some(defaults::get_sequence_start(
            increment.unwrap(),
            min_value.unwrap(),
            max_value.unwrap(),
        ));
    }
    if cache.is_none() {
        cache = Some(defaults::get_sequence_cache());
    }
    if cycle.is_none() {
        cycle = Some(defaults::get_sequence_cycle());
    }

    if !errors.is_empty() {
        return Err(Error::new_has_errors(n, errors));
    }

    Ok(Sequence {
        schema_name: ctx.schema_name.clone(),
        name: name,
        data_type: data_type.unwrap(),
        increment: increment.unwrap(),
        min_value: min_value.unwrap(),
        max_value: max_value.unwrap(),
        start: start.unwrap(),
        cache: cache.unwrap(),
        cycle: cycle.unwrap(),
        owned_by_table: owned_by_table.clone(),
        owned_by_column: owned_by_column.clone(),
    })
}

pub fn render_sequence_definition(_ctx: &Context, def: &Sequence) -> Node {
    let mut subnodes: Vec<Node> = Vec::new();

    if def.data_type != defaults::get_sequence_data_type() {
        subnodes.push(Node::new("as").with_string_value(def.data_type.clone()));
    }
    if def.increment != defaults::get_sequence_increment() {
        subnodes.push(Node::new("increment").with_number_integer_value(def.increment));
    }
    if def.min_value != defaults::get_sequence_min_value(def.increment, &def.data_type) {
        subnodes.push(Node::new("minvalue").with_number_integer_value(def.min_value));
    }
    if def.max_value != defaults::get_sequence_max_value(def.increment, &def.data_type) {
        subnodes.push(Node::new("maxvalue").with_number_integer_value(def.max_value));
    }
    if def.start != defaults::get_sequence_start(def.increment, def.min_value, def.max_value) {
        subnodes.push(Node::new("start").with_number_integer_value(def.start));
    }
    if def.cache != defaults::get_sequence_cache() {
        subnodes.push(Node::new("cache").with_number_integer_value(def.cache));
    }
    if def.cycle != defaults::get_sequence_cycle() {
        subnodes.push(Node::new("cycle").with_boolean_value(def.cycle));
    }
    if def.owned_by_table.is_some() && def.owned_by_table.is_some() {
        subnodes.push(
            Node::new("owned by")
                .with_name(def.owned_by_table.as_ref().unwrap().clone())
                .with_string_value(def.owned_by_column.as_ref().unwrap().clone()),
        );
    }

    Node::new("sequence")
        .with_name(def.name.clone())
        .with_nodes(subnodes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_and_render_sequence_definition() {
        let node = Node::new("sequence")
            .with_name("test".to_owned())
            .with_nodes(vec![
                Node::new("as").with_string_value("int4".to_owned()),
                Node::new("increment").with_number_integer_value(2),
                Node::new("minvalue").with_number_integer_value(0),
                Node::new("maxvalue").with_number_integer_value(100),
                Node::new("start").with_number_integer_value(2),
                Node::new("cache").with_number_integer_value(2),
                Node::new("cycle").with_boolean_value(true),
                Node::new("owned by")
                    .with_name("user".to_owned())
                    .with_string_value("id".to_owned()),
            ]);
        let def = Sequence {
            schema_name: "public".to_owned(),
            name: "test".to_owned(),
            data_type: "int4".to_owned(),
            increment: 2,
            min_value: 0,
            max_value: 100,
            start: 2,
            cache: 2,
            cycle: true,
            owned_by_table: Some("user".to_owned()),
            owned_by_column: Some("id".to_owned()),
        };
        let ctx = Context {
            schema_name: "public".to_owned(),
            table_name: None,
        };

        let got_def = parse_sequence_definition(&ctx, &node).unwrap();
        assert_eq!(got_def, def);
        let got_node = render_sequence_definition(&ctx, &def);
        assert_eq!(got_node, node);
    }
}
