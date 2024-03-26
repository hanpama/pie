#[cfg(test)]
use crate::{
    definition::load_yaml_string,
    snapshot::{
        changes::{AddPrimaryKeyChange, Change, DropPrimaryKeyChange},
        tests::utils::run_snapshot_diffing_test,
    },
};

#[test]
fn test_add_primary_key_change() {
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

                    constraint user_pkey:
                        primary key: id",
    )
    .unwrap();

    let expected = vec![Change::AddPrimaryKeyChange(AddPrimaryKeyChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        constraint: "user_pkey".to_string(),
        columns: vec!["id".to_string()],
        deferrable: false,
        initially_deferred: false,
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_drop_primary_key_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text

                    constraint user_pkey:
                        primary key: id",
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

    let expected = vec![Change::DropPrimaryKeyChange(DropPrimaryKeyChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        constraint: "user_pkey".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}
