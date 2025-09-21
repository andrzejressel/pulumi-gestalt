use anyhow::Context;
use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GitHubWorkflow {
    pub jobs: HashMap<String, Job>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Job {
    pub name: Option<String>,
    pub strategy: Option<Strategy>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Strategy {
    pub matrix: Option<HashMap<String, Vec<String>>>,
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
        let mut job_names = Vec::new();

        for (job_name, job) in &self.jobs {
            let job_name = job.name.clone().unwrap_or_else(|| job_name.clone());
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

    fn generate_matrix_combinations(&self, matrix: &HashMap<String, Vec<String>>) -> Vec<String> {
        matrix
            .values()
            .multi_cartesian_product()
            .map(|t| t.into_iter().join(" "))
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
    job: Build no matrix
    runs-on: ubuntu-latest
"#;

    #[test]
    fn test_parse_workflow() {
        let workflow = GitHubWorkflow::from_yaml(YAML_CONTENT).unwrap();

        let expected = GitHubWorkflow {
            jobs: HashMap::from([
                (
                    "build-base".to_string(),
                    Job {
                        name: None,
                        strategy: Some(Strategy {
                            matrix: Some(HashMap::from([(
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
                            matrix: Some(HashMap::from([(
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
            "build-base (macos-14)",
            "build-base (ubuntu-24.04)",
            "build-base (windows-2022)",
            "build-generated-provider (aws-0)",
            "build-generated-provider (aws-1)",
            "build-generated-provider (cloudflare)",
            "build-generated-provider (docker)",
            "Build no matrix",
        ];
    }
}
