#[cfg(test)]
use crate::{
    definition::load_yaml_string,
    snapshot::{
        changes::{
            AddColumnChange, AlterColumnSetDataTypeChange, AlterColumnSetDefaultChange,
            AlterColumnSetNotNullChange, Change, DropColumnChange,
        },
        tests::utils::run_snapshot_diffing_test,
    },
};

#[test]
fn test_add_column_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text
                    column email: text",
    )
    .unwrap();

    let expected = vec![Change::AddColumnChange(AddColumnChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        column: "email".to_string(),
        data_type: "text".to_string(),
        default: None,
        not_null: false,
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_alter_column_set_data_type_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: varchar",
    )
    .unwrap();

    let expected = vec![Change::AlterColumnSetDataTypeChange(
        AlterColumnSetDataTypeChange {
            schema: "public".to_string(),
            table: "user".to_string(),
            column: "username".to_string(),
            data_type: "varchar".to_string(),
        },
    )];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_alter_column_set_default_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text = 'anonymous'",
    )
    .unwrap();

    let expected = vec![Change::AlterColumnSetDefaultChange(
        AlterColumnSetDefaultChange {
            schema: "public".to_string(),
            table: "user".to_string(),
            column: "username".to_string(),
            default: Some("'anonymous'".to_string()),
        },
    )];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_alter_column_set_default_change_drop_default_case() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text = 'anonymous'",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text",
    )
    .unwrap();

    let expected = vec![Change::AlterColumnSetDefaultChange(
        AlterColumnSetDefaultChange {
            schema: "public".to_string(),
            table: "user".to_string(),
            column: "username".to_string(),
            default: None,
        },
    )];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_alter_column_set_not_null_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text!",
    )
    .unwrap();

    let expected = vec![Change::AlterColumnSetNotNullChange(
        AlterColumnSetNotNullChange {
            schema: "public".to_string(),
            table: "user".to_string(),
            column: "username".to_string(),
            not_null: true,
        },
    )];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_drop_column_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid",
    )
    .unwrap();

    let expected = vec![Change::DropColumnChange(DropColumnChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        column: "username".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}
