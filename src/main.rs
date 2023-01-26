mod nodejs;

use crate::nodejs::build_nodejs_readme;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    if !Path::new("package.json").is_file() {
        eprintln!("No package.json found");
        return ExitCode::FAILURE;
    }

    match build_nodejs_readme() {
        Ok(readme) => println!("{readme}"),
        Err(error) => {
            eprintln!("Could not build readme: {error}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
