use anyhow::Context;
use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GitHubWorkflow {
    pub jobs: BTreeMap<String, Job>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Job {
    pub name: Option<String>,
    pub strategy: Option<Strategy>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Strategy {
    pub matrix: Option<BTreeMap<String, Vec<String>>>,
}

impl GitHubWorkflow {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file {}", path))?;
        let workflow: GitHubWorkflow =
            Self::from_yaml(&content).with_context(|| format!("Failed to parse file {}", path))?;
        Ok(workflow)
    }

    pub fn from_yaml(yaml_content: &str) -> Result<Self> {
        serde_yaml::from_str(yaml_content).context("Failed to parse YAML content")
    }

    pub fn get_job_full_names(&self) -> Vec<String> {
        self.get_job_full_names_excluding_prefix("")
    }

    pub fn get_job_full_names_excluding_prefix(&self, prefix: &str) -> Vec<String> {
        let mut job_names = Vec::new();

        for (job_key, job) in &self.jobs {
            if !prefix.is_empty() && job_key.starts_with(prefix) {
                continue;
            }
            let job_name = job.name.clone().unwrap_or_else(|| job_key.clone());
            if let Some(matrix) = job.strategy.as_ref().and_then(|s| s.matrix.as_ref())
                && !matrix.is_empty()
            {
                let combinations = self.generate_matrix_combinations(matrix);
                for combo in combinations {
                    job_names.push(format!("{} ({})", job_name, combo));
                }
            } else {
                // Job without matrix
                job_names.push(job_name.clone());
            }
        }

        job_names.sort();
        job_names
    }

    fn generate_matrix_combinations(&self, matrix: &BTreeMap<String, Vec<String>>) -> Vec<String> {
        matrix
            .values()
            .multi_cartesian_product()
            .map(|t| t.into_iter().join(", "))
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const YAML_CONTENT: &str = r#"
jobs:
  build-base:
    strategy:
      matrix:
        os: [ubuntu-24.04, windows-2022, macos-14]
  build-generated-provider:
    strategy:
      matrix:
        provider: [cloudflare, docker, aws-0, aws-1]
  build-no-matrix:
    name: Build no matrix
    runs-on: ubuntu-latest
"#;

    #[test]
    fn test_parse_workflow() {
        let workflow = GitHubWorkflow::from_yaml(YAML_CONTENT).unwrap();

        let expected = GitHubWorkflow {
            jobs: BTreeMap::from([
                (
                    "build-base".to_string(),
                    Job {
                        name: None,
                        strategy: Some(Strategy {
                            matrix: Some(BTreeMap::from([(
                                "os".to_string(),
                                vec![
                                    "ubuntu-24.04".to_string(),
                                    "windows-2022".to_string(),
                                    "macos-14".to_string(),
                                ],
                            )])),
                        }),
                    },
                ),
                (
                    "build-generated-provider".to_string(),
                    Job {
                        name: None,
                        strategy: Some(Strategy {
                            matrix: Some(BTreeMap::from([(
                                "provider".to_string(),
                                vec![
                                    "cloudflare".to_string(),
                                    "docker".to_string(),
                                    "aws-0".to_string(),
                                    "aws-1".to_string(),
                                ],
                            )])),
                        }),
                    },
                ),
                (
                    "build-no-matrix".to_string(),
                    Job {
                        name: Some("Build no matrix".to_string()),
                        strategy: None,
                    },
                ),
            ]),
        };

        assert_eq!(workflow, expected);
    }

    #[test]
    fn test_generate_full_job_names() {
        let workflow = GitHubWorkflow::from_yaml(YAML_CONTENT).unwrap();

        let job_names = workflow.get_job_full_names();

        let expected_job_names = [
            "Build no matrix",
            "build-base (macos-14)",
            "build-base (ubuntu-24.04)",
            "build-base (windows-2022)",
            "build-generated-provider (aws-0)",
            "build-generated-provider (aws-1)",
            "build-generated-provider (cloudflare)",
            "build-generated-provider (docker)",
        ];

        assert_eq!(job_names, expected_job_names);
    }

    #[test]
    fn test_generate_full_job_names_excluding_prefix() {
        let workflow = GitHubWorkflow::from_yaml(YAML_CONTENT).unwrap();

        let job_names = workflow.get_job_full_names_excluding_prefix("build-generated");

        let expected_job_names = [
            "Build no matrix",
            "build-base (macos-14)",
            "build-base (ubuntu-24.04)",
            "build-base (windows-2022)",
        ];

        assert_eq!(job_names, expected_job_names);
    }

    #[test]
    fn test_matrix_with_multiple_keys() {
        const YAML_MULTI_KEY: &str = r#"
jobs:
  build-multi-matrix:
    strategy:
      matrix:
        os: [ubuntu-24.04, windows-2022]
        rust: [1.70, 1.71]
"#;

        let workflow = GitHubWorkflow::from_yaml(YAML_MULTI_KEY).unwrap();

        let job_names = workflow.get_job_full_names();

        let expected_job_names = [
            "build-multi-matrix (ubuntu-24.04, 1.70)",
            "build-multi-matrix (ubuntu-24.04, 1.71)",
            "build-multi-matrix (windows-2022, 1.70)",
            "build-multi-matrix (windows-2022, 1.71)",
        ];

        assert_eq!(job_names, expected_job_names);
    }
}
