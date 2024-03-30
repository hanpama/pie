pub fn get_constraint_deferrable() -> bool {
    false
}

pub fn get_constraint_initially_deferred() -> bool {
    false
}

pub fn get_foreign_key_match_option() -> String {
    "SIMPLE".to_string()
}

pub fn get_foreign_key_update_rule() -> String {
    "NO ACTION".to_string()
}

pub fn get_foreign_key_delete_rule() -> String {
    "NO ACTION".to_string()
}

pub fn get_index_unique() -> bool {
    false
}

pub fn get_default_index_method() -> String {
    "btree".to_string()
}

pub fn get_default_index_order() -> String {
    "ASC".to_string()
}

pub fn get_default_index_asc_nulls() -> String {
    "LAST".to_string()
}

pub fn get_default_index_desc_nulls() -> String {
    "FIRST".to_string()
}

pub fn get_default_column_default() -> String {
    "".to_string()
}

pub fn get_default_description() -> String {
    "".to_string()
}

pub fn get_function_language() -> String {
    "SQL".to_string()
}

pub fn get_function_returns() -> String {
    "void".to_string()
}

pub fn get_function_volatility() -> String {
    "VOLATILE".to_string()
}

pub fn get_sequence_data_type() -> String {
    "int8".to_string()
}

pub fn get_sequence_increment() -> i64 {
    1
}

pub fn get_sequence_cache() -> i64 {
    1
}

pub fn get_sequence_cycle() -> bool {
    false
}

pub fn get_sequence_min_value(increment: i64, data_type: &str) -> i64 {
    if increment >= 0 {
        return 1;
    }
    return match data_type {
        "int2" => -32768,
        "int4" => -2147483648,
        "int8" => -9223372036854775808,
        _ => panic!(r#"invalid sequence datatype: "{}""#, data_type),
    };
}

pub fn get_sequence_max_value(increment: i64, data_type: &str) -> i64 {
    if increment < 0 {
        return -1;
    }
    return match data_type {
        "int2" => 32767,
        "int4" => 2147483647,
        "int8" => 9223372036854775807,
        _ => panic!(r#"invalid sequence datatype: "{}""#, data_type),
    };
}

pub fn get_sequence_start(increment: i64, min_value: i64, max_value: i64) -> i64 {
    if increment >= 0 {
        min_value
    } else {
        max_value
    }
}
