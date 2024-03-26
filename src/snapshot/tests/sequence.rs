#[cfg(test)]
use crate::{
    definition::load_yaml_string,
    snapshot::{
        changes::{Change, CreateSequenceChange, DropSequenceChange},
        tests::utils::run_snapshot_diffing_test,
    },
};

#[test]
fn test_create_sequence_change() {
    let source = load_yaml_string(
        "
            schema public:",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:
                sequence user_id_seq:
                    as: int8",
    )
    .unwrap();

    let expected = vec![Change::CreateSequenceChange(CreateSequenceChange {
        schema: "public".to_string(),
        sequence: "user_id_seq".to_string(),
        data_type: "int8".to_string(),
        start: 1,
        increment: 1,
        min_value: 1,
        max_value: 9223372036854775807,
        cycle: false,
        cache: 1,
    })];

    run_snapshot_diffing_test(source, target, expected);
}

#[test]
fn test_drop_sequence_change() {
    let source = load_yaml_string(
        "
            schema public:
                sequence user_id_seq:
                    as: int8",
    )
    .unwrap();

    let target = load_yaml_string(
        "
            schema public:",
    )
    .unwrap();

    let expected = vec![Change::DropSequenceChange(DropSequenceChange {
        schema: "public".to_string(),
        sequence: "user_id_seq".to_string(),
    })];

    run_snapshot_diffing_test(source, target, expected);
}
