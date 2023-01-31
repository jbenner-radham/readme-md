use github::{get_github_workflows, GithubWorkflowBadges};
use license::get_license_section_body;
use md_writer::{fenced_rs_code_block, fenced_sh_code_block, h1, h2, LF};
use readme::{Readme, Section};
use std::env;
use std::fs;
use toml::{de::Error, map::Map, Value};

fn is_application_project() -> bool {
    let cwd = env::current_dir()
        .expect("Should have been able to read the current directory");

    cwd.join("src").join("main.rs").is_file()
}

pub fn parse_cargo_toml() -> Result<Value, Error> {
    let contents = fs::read_to_string("Cargo.toml")
        .expect("Should have been able to read the Cargo.toml file");
    let cargo = toml::from_str(&contents)?;

    Ok(cargo)
}

pub fn build_rust_readme(cargo: &Value) -> Result<String, Error> {
    let null_value = Value::Table(Map::new());
    let package = match cargo.get("package") {
        Some(package) => package,
        None => &null_value,
    };
    let name = match package.get("name") {
        Some(name) => name.as_str().unwrap().to_string(),
        None => String::from("<PACKAGE NAME>"),
    };
    let description = match package.get("description") {
        Some(description) => description.as_str().unwrap().to_string(),
        None => String::new(),
    };
    let license = match package.get("license") {
        Some(license) => license.as_str().unwrap().to_string(),
        None => String::new(),
    };
    let repository = match package.get("repository") {
        Some(repository) => repository.as_str().unwrap().to_string(),
        None => String::new(),
    };
    let github_workflows = get_github_workflows();
    let description = if repository.contains("github.com") && !github_workflows.is_empty() {
        let badges = GithubWorkflowBadges::new(&repository, &github_workflows);
        format!("{badges}{LF}{LF}{description}")
    } else {
        description
    };
    let header_section = Section {
        title: h1(&name),
        body: description,
    };
    let install_command = if is_application_project() { "install" } else { "add" };
    let install_section = Section {
        title: h2("Install"),
        body: fenced_sh_code_block(&format!("cargo {install_command} {name}")),
    };
    let usage_section_body = if is_application_project() {
        fenced_sh_code_block("# To be documented.")
    } else {
        fenced_rs_code_block("// To be documented.")
    };
    let usage_section = Section {
        title: h2("Usage"),
        body: usage_section_body,
    };
    let mut sections = vec![header_section, install_section, usage_section];

    if !license.is_empty() {
        let license_section = Section {
            title: h2("License"),
            body: get_license_section_body(&license),
        };
        sections.push(license_section);
    }

    let readme = Readme::new(sections);

    Ok(readme.to_string())
}
