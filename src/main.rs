mod github;
mod license;
mod node_js;
mod readme;
mod rust;

use crate::node_js::{build_node_js_readme, parse_package_json};
use crate::rust::{build_rust_readme, parse_cargo_toml};
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    // Is a Node.js project.
    if Path::new("package.json").is_file() {
        return match parse_package_json() {
            Ok(package) => match build_node_js_readme(&package) {
                Ok(readme) => {
                    println!("{readme}");
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("Could not build readme: {error}");
                    ExitCode::FAILURE
                }
            },
            Err(error) => {
                eprintln!("Could not parse package.json: {error}");
                ExitCode::FAILURE
            }
        };
    }

    // Is a Rust project.
    if Path::new("Cargo.toml").is_file() {
        return match parse_cargo_toml() {
            Ok(cargo) => match build_rust_readme(&cargo) {
                Ok(readme) => {
                    println!("{readme}");
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("Could not build readme: {error}");
                    ExitCode::FAILURE
                }
            },
            Err(error) => {
                eprintln!("Could not parse Cargo.toml: {error}");
                ExitCode::FAILURE
            }
        };
    }

    eprintln!("No supported project type found.");
    ExitCode::FAILURE
}
