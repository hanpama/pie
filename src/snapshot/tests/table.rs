#[cfg(test)]
use crate::{
    definition::load_yaml_string,
    snapshot::{
        changes::{Change, CreateTableChange, CreateTableChangeColumn, DropTableChange},
        tests::utils::run_snapshot_diffing_test,
    },
};

#[test]
fn test_create_table_change() {
    let source = load_yaml_string(
        "
            schema public:",
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

    let expected = vec![Change::CreateTableChange(CreateTableChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        columns: vec![
            CreateTableChangeColumn {
                name: "id".to_string(),
                data_type: "uuid".to_string(),
                default: None,
                not_null: false,
            },
            CreateTableChangeColumn {
                name: "username".to_string(),
                data_type: "text".to_string(),
                default: None,
                not_null: false,
            },
        ],
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_drop_table_change() {
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
            schema public:",
    )
    .unwrap();

    let expected = vec![Change::DropTableChange(DropTableChange {
        schema: "public".to_string(),
        table: "user".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}
