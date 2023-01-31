use md_writer::LF;
use std::fmt::{self, Formatter};
use std::path::Path;
use titlecase::titlecase;

pub struct GithubWorkflowBadge {
    alt_text: String,
    image_url: String,
    workflow_url: String,
}

impl GithubWorkflowBadge {
    pub fn new(repo_url: &str, workflow: &str) -> Self {
        let repo_url = match repo_url.strip_suffix('/') {
            Some(stripped_url) => stripped_url,
            None => repo_url,
        };
        GithubWorkflowBadge {
            alt_text: get_alt_text(workflow),
            image_url: format!("{repo_url}/actions/workflows/{workflow}/badge.svg"),
            workflow_url: format!("{repo_url}/actions/workflows/{workflow}"),
        }
    }
}

impl fmt::Display for GithubWorkflowBadge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[![{}]({})]({})",
            self.alt_text, self.image_url, self.workflow_url
        )
    }
}

pub struct GithubWorkflowBadges {
    badges: Vec<GithubWorkflowBadge>,
}

impl fmt::Display for GithubWorkflowBadges {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let badges = self
            .badges
            .iter()
            .map(|badge| badge.to_string())
            .collect::<Vec<_>>()
            .join(&LF.to_string());

        write!(f, "{badges}")
    }
}

impl GithubWorkflowBadges {
    pub fn new(repo_url: &str, workflows: &[String]) -> Self {
        let badges = workflows
            .iter()
            .map(|workflow| GithubWorkflowBadge::new(repo_url, workflow))
            .collect();

        Self { badges }
    }
}

pub fn get_github_workflows() -> Vec<String> {
    let path = Path::new(".github").join("workflows");
    let mut workflows: Vec<String> = vec![];

    if !path.is_dir() {
        return workflows;
    }

    for entry in path.read_dir().expect("read_dir call failed").flatten() {
        let file_name = entry.file_name().into_string().unwrap_or_else(|_| String::new());
        if file_name.ends_with(".yaml") || file_name.ends_with(".yml") {
            workflows.push(file_name);
        }
    }

    workflows
}

fn get_alt_text(workflow: &str) -> String {
    let workflow_basename = remove_yaml_file_extension(workflow);
    let uppercase_workflow_basename = workflow_basename.to_uppercase();

    if uppercase_workflow_basename == "CI" {
        uppercase_workflow_basename
    } else {
        to_titlecase(&workflow_basename)
    }
}

fn remove_yaml_file_extension(filename: &str) -> String {
    filename.replace(".yaml", "").replace(".yml", "")
}

fn to_titlecase(string: &str) -> String {
    let mut humanized = String::new();

    for char in string.chars() {
        if char.is_uppercase() {
            humanized.push(' ');
        }

        humanized.push(char);
    }

    let humanized = humanized
        .replace(['-', '_'], " ")
        .trim()
        .to_string();

    titlecase(&humanized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_alt_text_changes_a_hyphenated_workflow_into_titlecase_text() {
        let workflow = "build-release.yaml";
        let alt_text = get_alt_text(&workflow);

        assert_eq!(alt_text, "Build Release");
    }

    #[test]
    fn get_alt_text_changes_a_snakecased_workflow_into_titlecase_text() {
        let workflow = "build_release.yaml";
        let alt_text = get_alt_text(&workflow);

        assert_eq!(alt_text, "Build Release");
    }

    #[test]
    fn get_alt_text_changes_a_camelcased_workflow_into_titlecase_text() {
        let workflow = "buildRelease.yaml";
        let alt_text = get_alt_text(&workflow);

        assert_eq!(alt_text, "Build Release");
    }

    #[test]
    fn get_alt_text_changes_a_single_word_workflow_into_titlecase_text() {
        let workflow = "build.yaml";
        let alt_text = get_alt_text(&workflow);

        assert_eq!(alt_text, "Build");
    }

    #[test]
    fn get_alt_text_changes_a_workflow_named_ci_into_titlecase_text() {
        let workflow = "ci.yaml";
        let alt_text = get_alt_text(&workflow);

        assert_eq!(alt_text, "CI");
    }
}
