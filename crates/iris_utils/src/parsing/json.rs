use crate::error::{IrisUtilError, ParseError};

pub fn parse_json(value: &str) -> Result<String, IrisUtilError> {
    let start = value
        .find('{')
        .ok_or_else(|| IrisUtilError::Parse(ParseError::InvalidSyntax))?;
    let end = value
        .rfind('}')
        .ok_or_else(|| IrisUtilError::Parse(ParseError::UnexpectedEoI))?;

    let json_str = &value[start..=end];

    Ok(json_str.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_test_json() {
        let json_string = r#"{ name: "HelloWorld",
        }"#;

        assert!(parse_json(json_string).is_ok(), "Failed to Parse JSON Text")
    }

    #[test]
    fn failed_test_json() {
        let json_string = r#"{ name: "HelloWorld",
        "#;

        assert!(
            parse_json(json_string).is_err(),
            "Failed to Parse JSON Text"
        )
    }
}
