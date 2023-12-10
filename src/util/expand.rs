use regex::Regex;
use std::env;

/// Expand environment variables in a string.
pub fn expand_envvar(s: &str) -> String {
    let re = Regex::new(r"\$(\w+|\{\w+\})").unwrap();
    let result = re.replace_all(s, |caps: &regex::Captures| {
        let var = &caps[1];
        let var_name = var.trim_matches(|c| c == '{' || c == '}');
        env::var(var_name).unwrap_or_else(|_| "".to_string())
    });
    result.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_envvar() {
        env::set_var("FOO", "bar");
        assert_eq!(expand_envvar("hello $FOO"), "hello bar");
        assert_eq!(expand_envvar("hello ${FOO}"), "hello bar");
        assert_eq!(expand_envvar("hello $FOO world"), "hello bar world");
        assert_eq!(expand_envvar("hello ${FOO} world"), "hello bar world");
        assert_eq!(expand_envvar("hello $FOO$FOO"), "hello barbar");
        assert_eq!(expand_envvar("hello ${FOO$FOO"), "hello ${FOObar");
    }
}
