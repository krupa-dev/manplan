pub trait Candidate {
    fn available_versions(&self) -> Vec<String>;
}

pub struct SdkManCandidate {
    pub name: String,
    pub output: String,
}

mod private {
    use regex::Regex;

    use crate::sdkman::candidate::SdkManCandidate;

    pub trait VersionParser {
        fn non_java_versions(&self) -> Vec<String>;
        fn java_versions(&self) -> Vec<String>;
    }

    impl VersionParser for SdkManCandidate {
        fn non_java_versions(&self) -> Vec<String> {
            let mut equals = 0;
            let mut version_table: Vec<Vec<String>> = Vec::new();
            let lines = self.output.lines();
            let equals_pattern = Regex::new("^=+$").unwrap();
            for line in lines {
                if equals_pattern.is_match(line) {
                    equals += 1;
                } else if equals == 2 {
                    let columns: Vec<&str> = line
                        .split_whitespace()
                        .filter(|word| *word != "*" && *word != ">")
                        .collect();

                    for (i, el) in columns.iter().enumerate() {
                        while version_table.len() <= i {
                            version_table.push(Vec::new());
                        }
                        version_table[i].push(el.to_string());
                    }
                }
            }
            version_table.into_iter().flatten().collect()
        }

        fn java_versions(&self) -> Vec<String> {
            let mut equals = 0;
            let mut dashes = 0;
            let lines = self.output.lines();
            let mut versions: Vec<String> = Vec::new();
            let equals_pattern = Regex::new("^=+$").unwrap();
            let dashes_pattern = Regex::new("^-+$").unwrap();
            for line in lines {
                if equals_pattern.is_match(line) {
                    equals += 1;
                } else if dashes_pattern.is_match(line) {
                    dashes += 1;
                } else if equals == 2 && dashes == 1 {
                    line.split_whitespace()
                        .filter(|word| *word != "*" && *word != ">")
                        .last()
                        .map(|word| versions.push(word.to_string()));
                }
            }
            versions
        }
    }
}

impl Candidate for SdkManCandidate {
    fn available_versions(&self) -> Vec<String> {
        use crate::sdkman::candidate::private::VersionParser;
        if self.name == "java" {
            self.java_versions()
        } else {
            self.non_java_versions()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_kotlin_versions_parsed_in_order() {
        let candidate = SdkManCandidate {
            name: "kotlin".to_string(),
            output:
                "================================================================================
Available Kotlin Versions
================================================================================
 > * 1.9.0               1.4.20              1.2.70              1.1.4
     1.8.20              1.4.10              1.2.61              1.1.3-2

================================================================================
+ - local version
* - installed
> - currently in use
================================================================================"
                    .to_string(),
        };
        let expected = vec![
            "1.9.0", "1.8.20", "1.4.20", "1.4.10", "1.2.70", "1.2.61", "1.1.4", "1.1.3-2",
        ];
        assert_eq!(candidate.available_versions(), expected);
    }

    #[test]
    fn sample_java_versions_parsed_in_order() {
        let candidate = SdkManCandidate {
            name: "java".to_string(),
            output:
                "================================================================================
Available Java Versions for macOS ARM 64bit
================================================================================
 Vendor        | Use | Version      | Dist    | Status     | Identifier
--------------------------------------------------------------------------------
 Corretto      |     | 21           | amzn    |            | 21-amzn
               |     | 20.0.2       | amzn    |            | 20.0.2-amzn
 Gluon         |     | 22.1.0.1.r17 | gln     |            | 22.1.0.1.r17-gln
               |     | 22.1.0.1.r11 | gln     |            | 22.1.0.1.r11-gln
 GraalVM CE    |     | 21           | graalce | installed  | 21-graalce
               |     | 20.0.2       | graalce |            | 20.0.2-graalce
               |     | 20.0.1       | graalce |            | 20.0.1-graalce
================================================================================
Omit Identifier to install default version 17.0.8.1-tem:
    $ sdk install java
Use TAB completion to discover available versions
    $ sdk install java [TAB]
Or install a specific version by Identifier:
    $ sdk install java 17.0.8.1-tem
Hit Q to exit this list view
================================================================================"
                    .to_string(),
        };
        let expected = vec![
            "21-amzn",
            "20.0.2-amzn",
            "22.1.0.1.r17-gln",
            "22.1.0.1.r11-gln",
            "21-graalce",
            "20.0.2-graalce",
            "20.0.1-graalce",
        ];
        assert_eq!(candidate.available_versions(), expected);
    }
}
