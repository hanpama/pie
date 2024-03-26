use crate::{
    definition::{
        base::Node,
        ddl::{
            coerce::{coerce_name, coerce_nodes},
            context::Context,
            function::parse_function_definition,
            index::parse_index_definition,
            sequence::parse_sequence_definition,
            table::parse_table_definition,
            view::parse_view_definition,
        },
    },
    snapshot::{Function, Relation, Schema},
};

use super::{
    error::Error, function::render_function_definition, index::render_index_definition,
    sequence::render_sequence_definition, table::render_table_definition,
    view::render_view_definition,
};

pub fn parse_schema_definition(n: &Node) -> Result<Schema, Error> {
    assert_eq!(n.r#type, "schema");

    let name = coerce_name(n)?;
    let child_nodes = coerce_nodes(n)?;
    let mut errors: Vec<Error> = Vec::new();

    let ctx = Context::new(name.clone());

    let mut relations: Vec<Relation> = vec![];
    let mut functions: Vec<Function> = vec![];

    for cn in child_nodes {
        if let Err(e) = match cn.r#type {
            "table" => {
                parse_table_definition(&name, cn).and_then(|def| Ok(relations.push(def.into())))
            }
            "view" => {
                parse_view_definition(&ctx, cn).and_then(|def| Ok(relations.push(def.into())))
            }
            "sequence" => {
                parse_sequence_definition(&ctx, cn).and_then(|def| Ok(relations.push(def.into())))
            }
            "index" => {
                parse_index_definition(&ctx, cn).and_then(|def| Ok(relations.push(def.into())))
            }
            "function" => {
                parse_function_definition(&ctx, cn).and_then(|def| Ok(functions.push(def)))
            }
            _ => Err(Error::new_unexpected_node(cn)),
        } {
            errors.push(e);
        }
    }

    if !errors.is_empty() {
        return Err(Error::new_has_errors(n, errors));
    }

    let mut schema = Schema::new(&name);
    for relation in relations {
        schema.add_relation(relation).unwrap();
    }
    for function in functions {
        schema.add_function(function).unwrap();
    }

    Ok(schema)
}

pub fn render_schema_definition(def: &Schema) -> Node {
    let mut subnodes: Vec<Node> = Vec::new();

    let ctx = &Context::new(def.name.clone());

    for relation in def.iter_relations() {
        match relation {
            Relation::Table(table) => {
                subnodes.push(render_table_definition(ctx, table));
            }
            Relation::View(view) => {
                subnodes.push(render_view_definition(ctx, view));
            }
            Relation::Index(index) => {
                subnodes.push(render_index_definition(ctx, index));
            }
            Relation::Sequence(sequence) => {
                subnodes.push(render_sequence_definition(ctx, sequence));
            }
        }
    }
    for function in def.iter_functions() {
        subnodes.push(render_function_definition(ctx, function));
    }

    return Node::new("schema")
        .with_name(def.name.clone())
        .with_nodes(subnodes);
}
