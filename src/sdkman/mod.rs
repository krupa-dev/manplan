use crate::sdkman::candidate::{Candidate, SdkManCandidate};
use std::fs::read_dir;
use std::io::Write;
use std::{env, io};

mod candidate;

pub trait ToolManager {
    fn installed_versions(&self, candidate: String) -> Vec<String>;
    fn available_versions(&self, candidate: String) -> Vec<String>;
    fn install(&self, candidate: String, version: String);
    fn uninstall(&self, candidate: String, version: String);
    fn set_default(&self, candidate: String, version: String);
}

pub struct SdkMan {
    pub dry_run: bool,
    pub no_uninstall: bool,
}

impl ToolManager for SdkMan {
    #[allow(clippy::needless_return)]
    fn installed_versions(&self, candidate: String) -> Vec<String> {
        return match env::var("SDKMAN_DIR") {
            Ok(dir) => {
                let base = format!("{}/candidates/{}", dir, candidate);
                if std::path::Path::new(&base).is_dir() {
                    read_dir(base)
                        .expect("Failed to read $SDKMAN_DIR")
                        .filter(|entry| entry.as_ref().unwrap().file_type().unwrap().is_dir())
                        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
                        .collect()
                } else {
                    Vec::new()
                }
            }
            Err(_) => {
                panic!("SDKMAN_DIR not set. Is SDKMAN installed?")
            }
        };
    }

    fn available_versions(&self, candidate: String) -> Vec<String> {
        let shell = env::var("SHELL").unwrap();
        let output = std::process::Command::new(shell)
            .arg("-l")
            .arg("-i")
            .arg("-c")
            .arg(format!("sdk list {}", candidate))
            .output()
            .expect("Failed to run the sdk list command. Is SDKMAN installed?");
        if output.status.success() {
            SdkManCandidate {
                name: candidate,
                output: String::from_utf8(output.stdout).unwrap(),
            }
            .available_versions()
        } else {
            panic!(
                "Failed to run sdk list: {} {}",
                String::from_utf8(output.stdout).unwrap(),
                String::from_utf8(output.stderr).unwrap()
            );
        }
    }

    fn install(&self, candidate: String, version: String) {
        let shell = env::var("SHELL").unwrap();
        let cmd = format!("sdk install {} {}", candidate, version);
        print!("{}: ", cmd);
        io::stdout().flush().unwrap();
        if !self.dry_run {
            let output = std::process::Command::new(shell)
                .arg("-l")
                .arg("-i")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("Error running sdk install command");
            if output.status.success() {
                println!("OK");
            } else {
                println!(
                    "Error: {} {}",
                    String::from_utf8(output.stdout).unwrap(),
                    String::from_utf8(output.stderr).unwrap()
                );
            }
        } else {
            println!("DRY-RUN");
        }
    }

    fn uninstall(&self, candidate: String, version: String) {
        let shell = env::var("SHELL").unwrap();
        let cmd = format!("sdk uninstall {} {}", candidate, version);
        print!("{}: ", cmd);
        io::stdout().flush().unwrap();
        if self.no_uninstall {
            println!("NO-UNINSTALL");
        } else if !self.dry_run {
            let output = std::process::Command::new(shell)
                .arg("-l")
                .arg("-i")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("Error running sdk install command");
            if output.status.success() {
                println!("OK");
            } else {
                println!(
                    "Error: {} {}",
                    String::from_utf8(output.stdout).unwrap(),
                    String::from_utf8(output.stderr).unwrap()
                );
            }
        } else {
            println!("DRY-RUN");
        }
    }

    fn set_default(&self, candidate: String, version: String) {
        let shell = env::var("SHELL").unwrap();
        let cmd = format!("sdk default {} {}", candidate, version);
        print!("{}: ", cmd);
        io::stdout().flush().unwrap();
        if !self.dry_run {
            let output = std::process::Command::new(shell)
                .arg("-l")
                .arg("-i")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("Error running sdk install command");
            if output.status.success() {
                println!("OK");
            } else {
                println!(
                    "Error: {} {}",
                    String::from_utf8(output.stdout).unwrap(),
                    String::from_utf8(output.stderr).unwrap()
                );
            }
        } else {
            println!("DRY-RUN");
        }
    }
}
