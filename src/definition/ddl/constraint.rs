use crate::{
    definition::{
        base::Node,
        ddl::{check::parse_check_constraint_definition, coerce::coerce_nodes},
    },
    snapshot::Constraint,
};

use super::{
    check::render_check_constraint_definition,
    context::Context,
    error::Error,
    foreignkey::{
        parse_foreign_key_constraint_definition, render_foreign_key_constraint_definition,
    },
    primarykey::{parse_primary_key_definition, render_primary_key_definition},
    unique::{parse_unique_definition, render_unique_definition},
};

pub fn parse_constraint_definition(
    schema_name: &str,
    table_name: &str,
    n: &Node,
) -> Result<Constraint, Error> {
    let child_nodes = coerce_nodes(n)?;

    for cn in child_nodes {
        match cn.r#type {
            "primary key" => {
                let def = parse_primary_key_definition(schema_name, table_name, n)?;
                return Ok(Constraint::PrimaryKey(def));
            }
            "unique" => {
                let def = parse_unique_definition(schema_name, table_name, n)?;
                return Ok(Constraint::Unique(def));
            }
            "foreign key" => {
                let def = parse_foreign_key_constraint_definition(schema_name, table_name, n)?;
                return Ok(Constraint::ForeignKey(def));
            }
            "check" => {
                let def = parse_check_constraint_definition(schema_name, table_name, n)?;
                return Ok(Constraint::Check(def));
            }
            _ => unreachable!(),
        }
    }
    unreachable!()
}

pub fn render_constraint_definition(ctx: &Context, c: &Constraint) -> Node {
    match c {
        Constraint::PrimaryKey(def) => render_primary_key_definition(ctx, def),
        Constraint::Unique(def) => render_unique_definition(ctx, def),
        Constraint::ForeignKey(def) => render_foreign_key_constraint_definition(&ctx, def),
        Constraint::Check(def) => render_check_constraint_definition(&ctx, def),
    }
}
