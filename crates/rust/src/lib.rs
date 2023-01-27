use github::{get_github_workflows, GithubWorkflowBadges};
use md_writer::{fenced_rs_code_block, fenced_sh_code_block, h1, h2, LF};
use readme::{Readme, Section};
use std::fs;
use toml::{de::Error, map::Map, Value};

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
        Some(package) => package.as_str().unwrap().to_string(),
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
    let description = if repository.contains("github.com") && github_workflows.len() > 0 {
        let badges = GithubWorkflowBadges::new(&repository, &github_workflows).to_string();
        format!("{badges}{LF}{LF}{description}")
    } else {
        description.to_string()
    };
    let header_section = Section {
        title: h1(&name),
        body: description.to_string(),
    };
    let install_section = Section {
        title: h2("Install"),
        body: fenced_sh_code_block(&format!("cargo add {name}")),
    };
    let usage_section = Section {
        title: h2("Usage"),
        body: fenced_rs_code_block("// To be documented."),
    };
    let mut sections = vec![header_section, install_section, usage_section];

    if license.len() > 0 {
        let license_section = Section {
            title: h2("License"),
            body: format!("The {license} License. See the license file(s) for details."),
        };
        sections.push(license_section);
    }

    let readme = Readme::new(sections);

    Ok(readme.to_string())
}
