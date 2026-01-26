//! Unit tests for settings module

#[cfg(test)]
mod tests {
    use super::super::settings::*;
    use std::fs;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.max_recent, 10);
        assert!(settings.keywords.len() > 0);
    }

    #[test]
    fn test_settings_toml_parse() {
        let toml_content = r#"
max_recent = 15

[keywords]

[keywords.happy]
terms = ["happy", "joy"]
        "#;

        let settings: Settings = toml::from_str(toml_content).unwrap();
        assert_eq!(settings.max_recent, 15);
        assert!(settings.keywords.contains_key("happy"));
    }
}
