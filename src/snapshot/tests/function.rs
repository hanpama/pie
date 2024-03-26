#[cfg(test)]
use crate::{
    definition::load_yaml_string,
    snapshot::{
        changes::{Change, CreateFunctionChange, DropFunctionChange},
        tests::utils::run_snapshot_diffing_test,
    },
};

#[test]
fn test_create_function_change() {
    let source = load_yaml_string(
        "
            schema public:",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                function hello_world():
                    returns: text
                    language: sql
                    as: select 'Hello, World!';",
    )
    .unwrap();

    let expected = vec![Change::CreateFunctionChange(CreateFunctionChange {
        schema: "public".to_string(),
        function: "hello_world()".to_string(),
        returns: "text".to_string(),
        language: "sql".to_string(),
        body: "select 'Hello, World!';".to_string(),
        volatility: "VOLATILE".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_drop_function_change() {
    let source = load_yaml_string(
        "
            schema public:
                function hello_world():
                    returns: text
                    language: sql
                    as: select 'Hello, World!';",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:",
    )
    .unwrap();

    let expected = vec![Change::DropFunctionChange(DropFunctionChange {
        schema: "public".to_string(),
        function: "hello_world()".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}
