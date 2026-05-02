use crate::github::functions::get_repository::GetRepositoryArgs;
use crate::github::repository_ruleset::RepositoryRulesetArgs;
use crate::github::types::{
    RepositoryRulesetBypassActor, RepositoryRulesetConditions, RepositoryRulesetConditionsRefName,
    RepositoryRulesetRules, RepositoryRulesetRulesPullRequest,
    RepositoryRulesetRulesRequiredStatusChecksRequiredCheck,
};
use GithubIntegration::{Any, GithubActions, GithubAdvancedSecurity, ReleaserBot};
use anyhow::Result;
use github::issue_label::IssueLabelArgs;
use github::types::RepositoryRulesetRulesRequiredStatusChecks;
use pulumi_gestalt_rust::{Context, Output};
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
    ReleaserBot,
}

impl GithubIntegration {
    fn get_integration_id(&self) -> i32 {
        match self {
            Any => 0,
            GithubActions => 15368,
            GithubAdvancedSecurity => 57789,
            ReleaserBot => 3038871,
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
            .get_job_full_names_excluding_prefix("test-pulumi-language-rust-macos");
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

    create_labels(ctx, repository.id.clone());

    github::repository_ruleset::create(
        ctx,
        "main",
        RepositoryRulesetArgs::builder()
            .name("Main")
            .enforcement("active")
            .repository(repository.id)
            .target("branch")
            .bypass_actors(vec![
                RepositoryRulesetBypassActor::builder()
                    .actor_id(ReleaserBot.get_integration_id())
                    .actor_type("Integration")
                    .bypass_mode("always")
                    .build_struct(),
            ])
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
                    .pull_request(
                        RepositoryRulesetRulesPullRequest::builder()
                            .allowed_merge_methods(vec!["squash".to_string()])
                            .build_struct(),
                    )
                    .required_status_checks(
                        RepositoryRulesetRulesRequiredStatusChecks::builder()
                            .strict_required_status_checks_policy(false)
                            .required_checks(
                                [
                                    &[
                                        create_check("docs/readthedocs.org:pulumi-gestalt", Any),
                                        create_check("CodeQL", GithubAdvancedSecurity),
                                        create_check("clippy", GithubAdvancedSecurity),
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

fn create_labels(ctx: &Context, repository: Output<String>) {
    github::issue_label::create(
        ctx,
        "ready-to-merge-label",
        IssueLabelArgs::builder()
            .name("ready-to-merge")
            .color("0e8a16")
            .description("Will be auto-merged by mergify")
            .repository(repository.clone())
            .build_struct(),
    );

    github::issue_label::create(
        ctx,
        "conformance-tests-label",
        IssueLabelArgs::builder()
            .name("conformance-tests")
            .color("7393B3")
            .repository(repository.clone())
            .build_struct(),
    );

    github::issue_label::create(
        ctx,
        "conformance-tests-hard-label",
        IssueLabelArgs::builder()
            .name("conformance-tests/hard")
            .color("880808")
            .repository(repository.clone())
            .build_struct(),
    );

    github::issue_label::create(
        ctx,
        "conformance-tests-medium-label",
        IssueLabelArgs::builder()
            .name("conformance-tests/medium")
            .color("FDDA0D")
            .repository(repository.clone())
            .build_struct(),
    );

    github::issue_label::create(
        ctx,
        "conformance-tests-easy-label",
        IssueLabelArgs::builder()
            .name("conformance-tests/easy")
            .color("AAFF00")
            .repository(repository)
            .build_struct(),
    );
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
