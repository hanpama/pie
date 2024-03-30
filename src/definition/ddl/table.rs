use crate::{
    definition::{
        base::Node,
        ddl::coerce::{coerce_name, coerce_nodes},
    },
    snapshot::{Column, Constraint, Table},
};

use super::{
    column::{parse_column_definition, render_column_definition},
    constraint::{parse_constraint_definition, render_constraint_definition},
    context::Context,
    error::Error,
};

pub fn parse_table_definition(schema_name: &str, n: &Node) -> Result<Table, Error> {
    assert_eq!(n.r#type, "table");

    let name = coerce_name(n)?;
    let child_node = coerce_nodes(n)?;
    let mut errors: Vec<Error> = Vec::new();

    let mut columns: Vec<Column> = vec![];
    let mut constraints: Vec<Constraint> = vec![];

    for d in child_node {
        if let Err(e) = match d.r#type {
            "column" => {
                parse_column_definition(schema_name, &name, d).and_then(|def| Ok(columns.push(def)))
            }
            "constraint" => {
                parse_constraint_definition(schema_name, &name, d).and_then(|def| Ok(constraints.push(def)))
            }
            _ => Err(Error::new_unexpected_node(d)),
        } {
            errors.push(e);
        }
    }

    if !errors.is_empty() {
        return Err(Error::new_has_errors(n, errors));
    }

    let mut table = Table::new(schema_name, &name);
    for column in columns {
        table.add_column(column).unwrap();
    }
    for pk in constraints {
        table.add_constraint(pk).unwrap();
    }

    Ok(table)
}

pub fn render_table_definition(ctx: &Context, def: &Table) -> Node {
    let mut subnodes: Vec<Node> = Vec::new();

    for column in def.iter_columns() {
        subnodes.push(render_column_definition(ctx, column));
    }
    for constraint in def.iter_constraints() {
        subnodes.push(render_constraint_definition(ctx, constraint));
    }

    return Node::new("table")
        .with_name(def.name.clone())
        .with_nodes(subnodes);
}
