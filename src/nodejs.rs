use md_writer::{fenced_js_code_block, fenced_sh_code_block, h1, h2, LF};
use serde_json::{Result, Value};
use std::fmt::{self, Formatter};
use std::fs;
use std::path::Path;
use titlecase::titlecase;

struct Section {
    title: String,
    body: String,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{LF}{}", self.title, self.body)
    }
}

fn get_github_url(repository: &Value) -> String {
    if repository.is_object()
        && repository["url"].is_string()
        && repository["url"].as_str().unwrap().contains("github.com")
        && repository["type"].is_string()
        && repository["type"].as_str().unwrap() == "git"
    {
        return repository["url"]
            .as_str()
            .unwrap()
            .replace("git+", "")
            .replace(".git", "");
    }

    if repository.is_string() && repository.as_str().unwrap().contains("github.com") {
        return repository
            .as_str()
            .unwrap()
            .replace("git+", "")
            .replace(".git", "");
    }

    String::from("")
}

fn get_github_workflows() -> Vec<String> {
    let path = Path::new(".github").join("workflows");
    let mut workflows: Vec<String> = vec![];

    if !path.is_dir() {
        return workflows;
    }

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().into_string().unwrap_or("".to_string());
            if file_name.ends_with(".yaml") || file_name.ends_with(".yml") {
                workflows.push(file_name);
            }
        }
    }

    workflows
}

fn parse_package_json() -> Result<Value> {
    let contents = fs::read_to_string("package.json")
        .expect("Should have been able to read the package.json file");
    let package = serde_json::from_str(&contents)?;

    Ok(package)
}

fn remove_yaml_file_extension(filename: &str) -> String {
    filename.replace(".yaml", "").replace(".yml", "")
}

fn get_alt_text(workflow: &str) -> String {
    let workflow_basename = remove_yaml_file_extension(workflow);
    let uppercase_workflow_basename = workflow_basename.to_uppercase();

    if uppercase_workflow_basename == "CI" {
        uppercase_workflow_basename
    } else {
        titlecase(&workflow_basename)
    }
}

pub fn build_nodejs_readme() -> Result<String> {
    let package = parse_package_json().unwrap();
    let name = package["name"].as_str().unwrap_or("");
    let private = package["private"].as_bool().unwrap_or(false);
    let description = package["description"].as_str().unwrap_or("");
    let license = package["license"].as_str().unwrap_or("");
    let test_script = package["scripts"]["test"].as_str().unwrap_or("");
    let null_test_script = "echo \"Error: no test specified\" && exit 1";
    let has_test_script = test_script.len() > 0 && test_script != null_test_script;
    let github_workflows = get_github_workflows();
    let github_url = get_github_url(&package["repository"]);
    let description = if github_workflows.len() >= 1 && github_url.len() >= 1 {
        let badges = github_workflows
            .iter()
            .map(|workflow| {
                let alt_text = get_alt_text(&workflow);
                let workflow_image_url =
                    format!("{github_url}/actions/workflows/{workflow}/badge.svg");
                let workflow_url = format!("{github_url}/actions/workflows/{workflow}");
                format!("[![{alt_text}]({workflow_image_url})]({workflow_url})")
            })
            .collect::<Vec<String>>()
            .join(&LF.to_string());

        format!("{badges}{LF}{LF}{description}")
    } else {
        description.to_string()
    };
    let header_section = Section {
        title: h1(name),
        body: description.to_string(),
    };
    let usage_section = Section {
        title: h2("Usage"),
        body: fenced_js_code_block("// To be documented."),
    };
    let mut readme = vec![header_section];

    if !private {
        let install_section = Section {
            title: h2("Install"),
            body: fenced_sh_code_block(&format!("npm install {name}")),
        };
        readme.push(install_section);
    }

    readme.push(usage_section);

    if has_test_script {
        let testing_section = Section {
            title: h2("Testing"),
            body: fenced_sh_code_block("npm test"),
        };
        readme.push(testing_section);
    }

    if license.len() > 0 {
        let license_section = Section {
            title: h2("License"),
            body: if license.to_uppercase() == "UNLICENSED" {
                "This is unlicensed proprietary software.".to_string()
            } else {
                format!("The {license} License. See the license file(s) for details.")
            },
        };
        readme.push(license_section);
    }

    let readme = readme
        .iter()
        .map(|section| section.to_string())
        .collect::<Vec<String>>()
        .join(&LF.to_string().repeat(2));

    Ok(readme)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_github_url_parses_an_object() {
        let json = r#"
            {
                "type": "git",
                "url": "git+https://github.com/jbenner-radham/node-readme-md-cli.git"
            }
        "#;
        let repository = serde_json::from_str(json).unwrap();
        let url = get_github_url(&repository);

        assert_eq!(url, "https://github.com/jbenner-radham/node-readme-md-cli");
    }
}
