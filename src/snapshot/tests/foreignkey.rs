#[cfg(test)]
use crate::{
    definition::load_yaml_string,
    snapshot::{
        changes::{AddForeignKeyChange, AlterForeignKeyChange, Change, DropForeignKeyChange},
        tests::utils::run_snapshot_diffing_test,
    },
};

#[test]
fn test_add_foreign_key_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text
                    column friend_id: uuid",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text
                    column friend_id: uuid

                    constraint fk_user_friend_id:
                        foreign key: friend_id
                        references user: id",
    )
    .unwrap();

    let expected = vec![Change::AddForeignKeyChange(AddForeignKeyChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        constraint: "fk_user_friend_id".to_string(),
        columns: vec!["friend_id".to_string()],
        target_schema: "public".to_string(),
        target_table: "user".to_string(),
        target_columns: vec!["id".to_string()],
        deferrable: false,
        initially_deferred: false,
        match_option: "SIMPLE".to_string(),
        update_rule: "NO ACTION".to_string(),
        delete_rule: "NO ACTION".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_alter_foreign_key_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text
                    column friend_id: uuid

                    constraint fk_user_friend_id:
                        foreign key: friend_id
                        references user: id",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text
                    column friend_id: uuid

                    constraint fk_user_friend_id:
                        foreign key: friend_id
                        references user: id
                        deferrable: true",
    )
    .unwrap();

    let expected = vec![Change::AlterForeignKeyChange(AlterForeignKeyChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        constraint: "fk_user_friend_id".to_string(),
        deferrable: true,
        initially_deferred: false,
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_drop_foreign_key_change() {
    let source = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text
                    column friend_id: uuid

                    constraint fk_user_friend_id:
                        foreign key: friend_id
                        references user: id",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                table user:
                    column id: uuid
                    column username: text
                    column friend_id: uuid",
    )
    .unwrap();

    let expected = vec![Change::DropForeignKeyChange(DropForeignKeyChange {
        schema: "public".to_string(),
        table: "user".to_string(),
        constraint: "fk_user_friend_id".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}
