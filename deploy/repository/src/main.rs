use crate::github::functions::get_repository::GetRepositoryArgs;
use crate::github::repository_ruleset::RepositoryRulesetArgs;
use crate::github::types::{
    RepositoryRulesetConditions, RepositoryRulesetConditionsRefName, RepositoryRulesetRules,
    RepositoryRulesetRulesPullRequest, RepositoryRulesetRulesRequiredStatusChecksRequiredCheck,
};
use GithubIntegration::{Any, GithubActions, GithubAdvancedSecurity, Mergify};
use anyhow::Result;
use github::types::RepositoryRulesetRulesRequiredStatusChecks;
use pulumi_gestalt_rust::Context;
use std::ops::Deref;

mod github;
mod github_workflow;

fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}

enum GithubIntegration {
    Any,
    GithubActions,
    GithubAdvancedSecurity,
    Mergify,
}

impl GithubIntegration {
    fn get_integration_id(&self) -> i32 {
        match self {
            Any => 0,
            GithubActions => 15368,
            GithubAdvancedSecurity => 57789,
            Mergify => 10562,
        }
    }
}

fn pulumi_main(ctx: &Context) -> Result<()> {
    let repository = github::functions::get_repository::invoke(
        ctx,
        GetRepositoryArgs::builder()
            .name("pulumi-gestalt")
            .build_struct(),
    );

    let build_yaml_jobs =
        github_workflow::GitHubWorkflow::from_file("../../.github/workflows/build.yml")?
            .get_job_full_names();
    let codeql_yaml_jobs =
        github_workflow::GitHubWorkflow::from_file("../../.github/workflows/codeql.yml")?
            .get_job_full_names();
    let rust_clippy_jobs =
        github_workflow::GitHubWorkflow::from_file("../../.github/workflows/rust-clippy.yml")?
            .get_job_full_names();

    let all_jobs = [build_yaml_jobs, codeql_yaml_jobs, rust_clippy_jobs].concat();

    let pulumi_gestalt_checks: Vec<RepositoryRulesetRulesRequiredStatusChecksRequiredCheck> =
        all_jobs
            .iter()
            .map(|job_name| create_check(job_name, GithubActions))
            .collect();

    github::repository_ruleset::create(
        ctx,
        "main-protection-2",
        RepositoryRulesetArgs::builder()
            .name("main-protection-2")
            .enforcement("active")
            .repository(repository.id)
            .target("branch")
            .conditions(
                RepositoryRulesetConditions::builder()
                    .ref_name(
                        RepositoryRulesetConditionsRefName::builder()
                            .includes(vec!["~DEFAULT_BRANCH".to_string()])
                            .excludes(vec![])
                            .build_struct(),
                    )
                    .build_struct(),
            )
            .rules(
                RepositoryRulesetRules::builder()
                    .deletion(true)
                    .non_fast_forward(true)
                    .required_linear_history(true)
                    .pull_request(RepositoryRulesetRulesPullRequest::builder().build_struct())
                    .required_status_checks(
                        RepositoryRulesetRulesRequiredStatusChecks::builder()
                            .strict_required_status_checks_policy(false)
                            .required_checks(
                                [
                                    &[
                                        create_check("docs/readthedocs.org:pulumi-gestalt", Any),
                                        create_check("CodeQL", GithubAdvancedSecurity),
                                        create_check("clippy", GithubAdvancedSecurity),
                                        create_check("Mergify Merge Protections", Mergify),
                                    ],
                                    pulumi_gestalt_checks.deref(),
                                ]
                                .concat(),
                            )
                            .build_struct(),
                    )
                    .build_struct(),
            )
            .build_struct(),
    );

    Ok(())
}

fn create_check(
    name: &str,
    integration: GithubIntegration,
) -> RepositoryRulesetRulesRequiredStatusChecksRequiredCheck {
    RepositoryRulesetRulesRequiredStatusChecksRequiredCheck::builder()
        .context(name)
        .integration_id(integration.get_integration_id())
        .build_struct()
}
