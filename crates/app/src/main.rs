use nodejs::{build_nodejs_readme, parse_package_json};
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    if Path::new("package.json").is_file() {
        return match parse_package_json() {
            Ok(package) => match build_nodejs_readme(&package) {
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

    eprintln!("No supported project type found.");
    ExitCode::FAILURE
}
