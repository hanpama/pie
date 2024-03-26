#[cfg(test)]
use crate::{
    definition::load_yaml_string,
    snapshot::{
        changes::{AddUniqueChange, AlterUniqueChange, Change, DropUniqueChange},
        tests::utils::run_snapshot_diffing_test,
    },
};

#[test]
fn test_add_unique_change() {
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

                    constraint user_username_key:
                        unique: username",
    )
    .unwrap();

    let expected = vec![Change::AddUniqueChange(AddUniqueChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        constraint: "user_username_key".to_string(),
        columns: vec!["username".to_string()],
        deferrable: false,
        initially_deferred: false,
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_alter_unique_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text

                    constraint user_username_key:
                        unique: username",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text

                    constraint user_username_key:
                        unique: username
                        deferrable: true",
    )
    .unwrap();

    let expected = vec![Change::AlterUniqueChange(AlterUniqueChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        constraint: "user_username_key".to_string(),
        deferrable: true,
        initially_deferred: false,
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_drop_unique_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text

                    constraint user_username_key:
                        unique: username",
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

    let expected = vec![Change::DropUniqueChange(DropUniqueChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        constraint: "user_username_key".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}
