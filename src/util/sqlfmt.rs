// quoted name
pub fn sql_qn(schema: &str) -> String {
    format!("\"{}\"", schema)
}

// quoted accessor
pub fn sql_qa(schema: &str, relation: &str) -> String {
    format!("{}.{}", sql_qn(schema), sql_qn(relation))
}

// quoted list
pub fn sql_ql<I, S>(names: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let quoted_names = names
        .into_iter()
        .map(|name| sql_qn(name.as_ref()))
        .collect::<Vec<_>>();
    sql_l(&quoted_names)
}

// list
pub fn sql_l<I, S>(els: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    els.into_iter()
        .map(|s| s.as_ref().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
