use std::collections::HashSet;
use std::fs;

use clap::Parser;

use crate::rules::{parse_rules, VersionMatch};
use crate::sdkman::{SdkMan, ToolManager};

pub mod rules;
pub mod sdkman;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename of the YAML config file
    #[arg(short, long)]
    file: String,

    /// Just print out the commands that would be executed
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,

    /// Do not uninstall non-required candidate versions
    #[arg(short, long, default_value_t = false)]
    no_uninstall: bool,
}

fn main() {
    let args = Args::parse();

    let sdkman = SdkMan {
        dry_run: args.dry_run,
        no_uninstall: args.no_uninstall,
    };
    let rules = parse_rules(fs::read_to_string(args.file).expect("Failed to read input file"));
    for (name, candidate) in rules.expect("Rules file could not be parsed").candidates {
        let installed: HashSet<_> = sdkman
            .installed_versions(name.clone())
            .into_iter()
            .collect();
        let available = sdkman.available_versions(name.clone());
        let mut required: HashSet<String> = HashSet::new();
        let mut default: Option<String> = None;

        for version in candidate.versions {
            let the_match = version.get_matching(name.clone(), available.clone());
            the_match.map(|it| {
                required.insert(it.to_string());
                if version.default.unwrap_or(false) {
                    default = Some(it.to_string());
                }
            });
        }

        let to_install = required.difference(&installed);
        let to_remove = installed.difference(&required);

        for removal in to_remove {
            sdkman.uninstall(name.clone(), removal.to_string());
        }

        for installation in to_install {
            sdkman.install(name.clone(), installation.to_string());
        }

        if let Some(default) = default {
            sdkman.set_default(name.clone(), default);
        }
    }
}
