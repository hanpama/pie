#[cfg(test)]
use crate::{
    definition::load_yaml_string,
    snapshot::{
        changes::{Change, CreateSchemaChange, DropSchemaChange},
        tests::utils::run_snapshot_diffing_test,
    },
};

#[test]
fn test_create_schema_change() {
    let source = load_yaml_string(
        "
            schema public:",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
            schema test:",
    )
    .unwrap();

    let expected = vec![Change::CreateSchemaChange(CreateSchemaChange {
        schema_name: "test".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_drop_schema_change() {
    let source = load_yaml_string(
        "
            schema public:
            schema test:",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:",
    )
    .unwrap();

    let expected = vec![Change::DropSchemaChange(DropSchemaChange {
        schema_name: "test".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}
