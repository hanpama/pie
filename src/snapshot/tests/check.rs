#[cfg(test)]
use crate::{
    definition::load_yaml_string,
    snapshot::{
        changes::{AddCheckChange, Change, AlterCheckChange, DropCheckChange},
        tests::utils::run_snapshot_diffing_test,
    },
};

#[test]
fn test_add_check_chage() {
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
                    
                    constraint username_length:
                        check: length(username) > 5",
    )
    .unwrap();

    let expected = vec![Change::AddCheckChange(AddCheckChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        constraint: "username_length".to_string(),
        expression: "length(username) > 5".to_string(),
        deferrable: false,
        initially_deferred: false,
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_alter_check_chage() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text

                    constraint username_length:
                        check: length(username) > 5",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text

                    constraint username_length:
                        check: length(username) > 5
                        deferrable: true
                        initially deferred: true",
    )
    .unwrap();

    let expected = vec![Change::AlterCheckChange(AlterCheckChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        constraint: "username_length".to_string(),
        deferrable: true,
        initially_deferred: true,
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_drop_check_chage() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text

                    constraint username_length:
                        check: length(username) > 5",
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

    let expected = vec![Change::DropCheckChange(DropCheckChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        constraint: "username_length".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}
