#[cfg(test)]
use crate::{
    definition::load_yaml_string,
    snapshot::{
        changes::{Change, CreateIndexChange, DropIndexChange},
        tests::utils::run_snapshot_diffing_test,
    },
};

#[test]
fn test_create_index_change() {
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

                index user_username_idx:
                    on user: username",
    )
    .unwrap();

    let expected = vec![Change::CreateIndexChange(CreateIndexChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        index: "user_username_idx".to_string(),
        key_expressions: vec!["username".to_string()],
        method: "btree".to_string(),
        unique: false,
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_drop_index_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text

                index user_username_idx:
                    on user: username",
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

    let expected = vec![Change::DropIndexChange(DropIndexChange {
        schema: "public".to_string(),
        index: "user_username_idx".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}
