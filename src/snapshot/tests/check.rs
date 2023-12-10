#[cfg(test)]
use crate::{
    definition::load_yaml_string,
    snapshot::{
        changes::{AddCheckChange, Change},
        compare_diff,
    },
};

#[test]
fn test_add_check_chage() {
    let mut source = load_yaml_string(
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

    let changes = compare_diff(&source, &target);

    assert_eq!(
        changes,
        vec![Change::AddCheckChange(AddCheckChange {
            schema: "public".to_string(),
            table: "user".to_string(),
            constraint: "username_length".to_string(),
            expression: "length(username) > 5".to_string(),
            deferrable: false,
            initially_deferred: false,
        })]
    );

    changes.iter().for_each(|change| {
        change.apply(&mut source).unwrap();
    });

    let changes = compare_diff(&source, &target);

    assert_eq!(changes, vec![]);
}
