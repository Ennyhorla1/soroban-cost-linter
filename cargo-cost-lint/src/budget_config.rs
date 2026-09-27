use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct BudgetConfig {
    pub lints: Option<std::collections::HashMap<String, String>>,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
struct BudgetConfigWrapper {
    budget: Option<BudgetConfig>,
}

pub fn parse_config(config_path: &Path) -> BudgetConfig {
    if let Ok(contents) = fs::read_to_string(config_path) {
        parse_config_str(&contents)
    } else {
        BudgetConfig::default()
    }
}

pub fn parse_config_str(raw: &str) -> BudgetConfig {
    if raw.trim().is_empty() {
        return BudgetConfig::default();
    }
    if let Ok(wrapper) = toml::from_str::<BudgetConfigWrapper>(raw) {
        if let Some(budget) = wrapper.budget {
            return budget;
        }
    }
    toml::from_str::<BudgetConfig>(raw).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn write_file(path: &Path, contents: &str) {
        let mut f = File::create(path).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
    }

    #[test]
    fn empty_file_returns_defaults() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("budget.toml");
        write_file(&path, "");
        let config = parse_config(&path);
        assert!(config.lints.is_none());
    }

    #[test]
    fn whitespace_only_file_returns_defaults() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("budget.toml");
        write_file(&path, "   \n\t  ");
        let config = parse_config(&path);
        assert!(config.lints.is_none());
    }

    #[test]
    fn missing_file_returns_defaults() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.toml");
        let config = parse_config(&path);
        assert!(config.lints.is_none());
    }

    #[test]
    fn invalid_toml_returns_defaults() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("budget.toml");
        write_file(&path, "this is invalid toml [[[ ");
        let config = parse_config(&path);
        assert!(config.lints.is_none());
    }

    #[test]
    fn empty_string_returns_defaults() {
        let config = parse_config_str("");
        assert!(config.lints.is_none());
    }

    #[test]
    fn whitespace_only_string_returns_defaults() {
        let config = parse_config_str("   \n  ");
        assert!(config.lints.is_none());
    }

    #[test]
    fn invalid_toml_string_returns_defaults() {
        let config = parse_config_str("not = toml = format = error");
        assert!(config.lints.is_none());
    }

    #[test]
    fn missing_budget_table_returns_defaults() {
        let config = parse_config_str("[lints]\nsoroban_storage_in_loop = \"deny\"");
        let lints = config.lints.as_ref().unwrap();
        assert_eq!(lints.get("soroban_storage_in_loop").unwrap(), "deny");
    }

    #[test]
    fn missing_lints_field_returns_defaults() {
        let config = parse_config_str("[budget]\nother = 123");
        assert!(config.lints.is_none());
    }

    #[test]
    fn valid_config_parses_correctly() {
        let raw = r#"
            [budget.lints]
            soroban_storage_in_loop = "deny"
            redundant_env_clone = "warn"
        "#;
        let config = parse_config_str(raw);
        let lints = config.lints.as_ref().unwrap();
        assert_eq!(lints.get("soroban_storage_in_loop").unwrap(), "deny");
        assert_eq!(lints.get("redundant_env_clone").unwrap(), "warn");
    }

    #[test]
    fn valid_config_file_parses_correctly() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("budget.toml");
        write_file(&path, "[lints]\nredundant_address_clone = \"allow\"");
        let config = parse_config(&path);
        let lints = config.lints.as_ref().unwrap();
        assert_eq!(lints.get("redundant_address_clone").unwrap(), "allow");
    }
}
