use expect_exit::Expected;
use regex::Regex;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::{self, Error};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Version {
    pub pattern: String,
    pub default: Option<bool>,
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Candidate {
    pub versions: Vec<Version>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Rules {
    pub candidates: HashMap<String, Candidate>,
}

pub fn parse_rules(data: String) -> Result<Rules, Error> {
    serde_yaml::from_str(&data)
}

pub trait VersionMatch {
    fn get_matching(&self, name: String, available: Vec<String>) -> Option<String>;
}

impl VersionMatch for Version {
    fn get_matching(&self, name: String, available: Vec<String>) -> Option<String> {
        let pattern = Regex::new(self.pattern.as_str())
            .or_exit_(format!("Invalid regex for {}: {}", name, self.pattern).as_str());
        let mut matches: Vec<String> = available
            .iter()
            .filter(|it| pattern.is_match(it))
            .map(|it| it.to_string())
            .collect();

        if let Some(exclude) = self.exclude.as_ref() {
            let exclude_pattern = Regex::new(exclude.join("|").as_str())
                .or_exit_(format!("Invalid regex for {}: {}", name, exclude.join("|")).as_str());
            matches.retain(|it| !exclude_pattern.is_match(it));
        }

        matches.first().map(|it| it.to_string());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn sample_rules_parsed() {
        let input = "---
candidates:
  java:
    versions:
      - pattern: \"21.*-zulu\"
        default: true
      - pattern: \"21.*-graalce\"
  groovy:
    versions:
      - pattern: \".*\"
        exclude:
          - \".*alpha.*\"
          - \".*-rc.*\"
        default: true"
            .to_string();
        let expected: Rules = Rules {
            candidates: hashmap! {
                "java".to_string() => Candidate{
                    versions: vec![
                        Version{
                            pattern: "21.*-zulu".to_string(),
                            default: Some(true),
                            exclude: None
                        },
                        Version{
                            pattern: "21.*-graalce".to_string(),
                            default: None,
                            exclude: None
                        }
                    ]
                },
                "groovy".to_string() => Candidate{
                    versions: vec![
                        Version{
                            pattern: ".*".to_string(),
                            default: Some(true),
                            exclude: Some(vec![
                                ".*alpha.*".to_string(),
                                ".*-rc.*".to_string()
                            ])
                        }
                    ]
                }
            },
        };

        match parse_rules(input) {
            Ok(rules) => assert_eq!(expected, rules),
            Err(err) => {
                println!("Error: {}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn versions_matched() {
        let version = Version {
            pattern: "^21.*$".to_string(),
            default: None,
            exclude: Some(vec!["-zulu".to_string(), "-graalce".to_string()]),
        };
        let available = vec![
            "21.0.0-zulu".to_string(),
            "21.0.0-graalce".to_string(),
            "21.0.0-amzn".to_string(),
            "21.0.0".to_string(),
        ];
        assert_eq!(
            version.get_matching("java".to_string(), available),
            Some("21.0.0-amzn".to_string())
        );
    }

    #[test]
    fn versions_matched_with_no_exclusion() {
        let version = Version {
            pattern: "^21.*$".to_string(),
            default: None,
            exclude: None,
        };
        let available = vec![
            "21.0.0-zulu".to_string(),
            "21.0.0-graalce".to_string(),
            "21.0.0-amzn".to_string(),
            "21.0.0".to_string(),
        ];
        assert_eq!(
            version.get_matching("java".to_string(), available),
            Some("21.0.0-zulu".to_string())
        );
    }

    #[test]
    fn versions_not_matched() {
        let version = Version {
            pattern: "^11.*$".to_string(),
            default: None,
            exclude: None,
        };
        let available = vec![
            "21.0.0-zulu".to_string(),
            "21.0.0-graalce".to_string(),
            "21.0.0-amzn".to_string(),
            "21.0.0".to_string(),
        ];
        assert_eq!(version.get_matching("java".to_string(), available), None);
    }
}
