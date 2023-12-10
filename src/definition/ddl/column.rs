use crate::{
    definition::{base::Node, ddl::coerce::coerce_string_value},
    snapshot::Column,
};

use super::{coerce::coerce_name, context::Context, error::Error};
use regex::Regex;

pub fn parse_column_definition(schema: &str, table: &str, n: &Node) -> Result<Column, Error> {
    assert_eq!(n.r#type, "column");

    let name = coerce_name(n)?;
    let type_expr_str = coerce_string_value(n)?;
    let (data_type, not_null, default) = parse_column_expression(&type_expr_str)
        .or_else(|err| Err(Error::new_has_errors(n, vec![err])))?;

    return Ok(Column {
        schema_name: schema.to_owned(),
        table_name: table.to_owned(),
        name,
        data_type,
        not_null,
        default,
    });
}

pub fn render_column_definition(ctx: &Context, def: &Column) -> Node {
    let type_expr_str = render_column_expression(&def.data_type, def.not_null, &def.default);

    return Node::new("column")
        .with_name(def.name.clone())
        .with_string_value(type_expr_str);
}

fn parse_column_expression(input: &str) -> Result<(String, bool, Option<String>), Error> {
    let re = Regex::new(r"^(?P<type>[^=!]+)(?P<notnull>!)?(?:\s*=\s*(?P<default>.+))?$").unwrap();
    if let Some(caps) = re.captures(input) {
        let data_type = caps.name("type").unwrap().as_str().trim().to_string();
        let not_null = caps.name("notnull").is_some();
        let default_expr = caps.name("default").map(|s| s.as_str().trim().to_string());

        Ok((data_type, not_null, default_expr))
    } else {
        Err(Error::InvalidTypeExpression {
            type_expression: input.to_string(),
        })
    }
}

fn render_column_expression(
    data_type: &str,
    not_null: bool,
    default_expr: &Option<String>,
) -> String {
    let mut expr = data_type.to_string();
    if not_null {
        expr.push_str("!");
    }
    if let Some(default_str) = default_expr {
        expr.push_str(" = ");
        expr.push_str(&default_str);
    }
    expr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_column_expression() {
        assert_eq!(
            parse_column_expression("text").unwrap(),
            ("text".to_string(), false, None)
        );
        assert_eq!(
            parse_column_expression("text!").unwrap(),
            ("text".to_string(), true, None)
        );
        assert_eq!(
            parse_column_expression("text = 'hello!'").unwrap(),
            ("text".to_string(), false, Some("'hello!'".to_string()))
        );
        assert_eq!(
            parse_column_expression("text! = 'hello world!!'").unwrap(),
            (
                "text".to_string(),
                true,
                Some("'hello world!!'".to_string())
            )
        );
    }

    #[test]
    fn test_render_column_expression() {
        assert_eq!(
            render_column_expression("text", false, &None),
            "text".to_string()
        );
        assert_eq!(
            render_column_expression("text", true, &None),
            "text!".to_string()
        );
        assert_eq!(
            render_column_expression("text", false, &Some("'hello'".to_string())),
            "text = 'hello'".to_string()
        );
        assert_eq!(
            render_column_expression("text", false, &Some("'hello world!!'".to_string())),
            "text = 'hello world!!'".to_string()
        );
    }

    #[test]
    fn test_parse_and_render_column_definition() {
        let node = Node::new("column")
            .with_name("id".to_owned())
            .with_string_value("integer".to_owned());
        let def = Column {
            schema_name: "public".to_owned(),
            table_name: "user".to_owned(),
            name: "id".to_owned(),
            data_type: "integer".to_owned(),
            default: None,
            not_null: false,
        };
        let ctx = Context {
            schema_name: "public".to_owned(),
            table_name: Some("user".to_owned()),
        };

        let got_def = parse_column_definition("public", "user", &node).unwrap();
        assert_eq!(got_def, def);

        let got_node = render_column_definition(&ctx, &def);
        assert_eq!(got_node, node);
    }
}
