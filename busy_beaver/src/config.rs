use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub max_iterations: usize,
    pub programs: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config_deserialization() {
        let config_str = r#"
            max_iterations = 100

            programs = [
            "1RB1RZ_1LB0RC_1LC1LA",
            "1RB1RZ_0LC0RC_1LC1LA",
            "1RB1LA_0RC1RZ_1LC0LA",
            "1RB1RA_0RC1RZ_1LC0LA",
            "1RB0RA_0RC1RZ_1LC0LA",
            "1RB0LC_1LA1RZ_1RC1RB",
            "1RB0LB_0RC1RC_1LA1RZ",
            "1RB1LB_0RC1RZ_1LC0LA",
            "1RB0LB_0RC1RZ_1LC0LA",
            "1RB1RZ_0RC0RC_1LC1LA",
            ]
        "#;

        let config: Config = toml::from_str(config_str).unwrap();
        assert_eq!(config.max_iterations, 100);
        assert_eq!(config.programs.len(), 10);
        assert_eq!(config.programs[0], "1RB1RZ_1LB0RC_1LC1LA");
        assert_eq!(config.programs[1], "1RB1RZ_0LC0RC_1LC1LA");
    }
}
