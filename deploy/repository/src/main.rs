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

mod github;

pulumi_gestalt_rust::pulumi_main!();

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
                    .deletion(Box::new(Some(true)))
                    .non_fast_forward(Box::new(Some(true)))
                    .required_linear_history(Box::new(Some(true)))
                    .pull_request(Box::new(Some(
                        RepositoryRulesetRulesPullRequest::builder().build_struct(),
                    )))
                    .required_status_checks(Box::new(Some(
                        RepositoryRulesetRulesRequiredStatusChecks::builder()
                            .strict_required_status_checks_policy(Box::new(Some(false)))
                            .required_checks(Box::new(vec![
                                // Any
                                create_check("docs/readthedocs.org:pulumi-gestalt", Any),
                                // GithubActions
                                create_check("Analyze C", GithubActions),
                                create_check("Analyze Go", GithubActions),
                                create_check("build-base (ubuntu-24.04)", GithubActions),
                                create_check("build-base (windows-2022)", GithubActions),
                                create_check("build-c (ubuntu-24.04)", GithubActions),
                                create_check("build-c (windows-2022)", GithubActions),
                                create_check("build-examples (ubuntu-24.04)", GithubActions),
                                create_check("build-examples (windows-2022)", GithubActions),
                                create_check(
                                    "build-generated-provider (array-of-enum-map)",
                                    GithubActions,
                                ),
                                create_check("build-generated-provider (aws-0)", GithubActions),
                                create_check("build-generated-provider (aws-1)", GithubActions),
                                create_check("build-generated-provider (aws-2)", GithubActions),
                                create_check("build-generated-provider (aws-3)", GithubActions),
                                create_check("build-generated-provider (aws-4)", GithubActions),
                                create_check("build-generated-provider (aws-5)", GithubActions),
                                create_check("build-generated-provider (aws-6)", GithubActions),
                                create_check("build-generated-provider (aws-7)", GithubActions),
                                create_check("build-generated-provider (aws-8)", GithubActions),
                                create_check("build-generated-provider (aws-9)", GithubActions),
                                create_check("build-generated-provider (aws-10)", GithubActions),
                                create_check("build-generated-provider (aws-11)", GithubActions),
                                create_check("build-generated-provider (aws-12)", GithubActions),
                                create_check("build-generated-provider (aws-13)", GithubActions),
                                create_check("build-generated-provider (aws-14)", GithubActions),
                                create_check("build-generated-provider (aws-15)", GithubActions),
                                create_check("build-generated-provider (aws-16)", GithubActions),
                                create_check("build-generated-provider (aws-17)", GithubActions),
                                create_check("build-generated-provider (aws-18)", GithubActions),
                                create_check("build-generated-provider (aws-19)", GithubActions),
                                create_check("build-generated-provider (aws-20)", GithubActions),
                                create_check("build-generated-provider (aws-21)", GithubActions),
                                create_check("build-generated-provider (azure-0)", GithubActions),
                                create_check("build-generated-provider (azure-1)", GithubActions),
                                create_check("build-generated-provider (azure-2)", GithubActions),
                                create_check("build-generated-provider (azure-3)", GithubActions),
                                create_check("build-generated-provider (azure-4)", GithubActions),
                                create_check("build-generated-provider (azure-5)", GithubActions),
                                create_check("build-generated-provider (azure-6)", GithubActions),
                                create_check("build-generated-provider (azure-7)", GithubActions),
                                create_check("build-generated-provider (azure-8)", GithubActions),
                                create_check("build-generated-provider (azure-9)", GithubActions),
                                create_check("build-generated-provider (azure-10)", GithubActions),
                                create_check("build-generated-provider (azure-11)", GithubActions),
                                create_check("build-generated-provider (azure-12)", GithubActions),
                                create_check("build-generated-provider (azure-13)", GithubActions),
                                create_check(
                                    "build-generated-provider (cloudflare)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (cyclic-types)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (different-enum)",
                                    GithubActions,
                                ),
                                create_check("build-generated-provider (docker)", GithubActions),
                                create_check(
                                    "build-generated-provider (filtering-0)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (filtering-1)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (filtering-2)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (functions-secrets)",
                                    GithubActions,
                                ),
                                create_check("build-generated-provider (gcp-0)", GithubActions),
                                create_check("build-generated-provider (gcp-1)", GithubActions),
                                create_check("build-generated-provider (gcp-2)", GithubActions),
                                create_check("build-generated-provider (gcp-3)", GithubActions),
                                create_check("build-generated-provider (gcp-4)", GithubActions),
                                create_check("build-generated-provider (gcp-5)", GithubActions),
                                create_check("build-generated-provider (gcp-6)", GithubActions),
                                create_check("build-generated-provider (gcp-7)", GithubActions),
                                create_check("build-generated-provider (gcp-8)", GithubActions),
                                create_check("build-generated-provider (gcp-9)", GithubActions),
                                create_check("build-generated-provider (gcp-10)", GithubActions),
                                create_check("build-generated-provider (gcp-11)", GithubActions),
                                create_check("build-generated-provider (gcp-12)", GithubActions),
                                create_check("build-generated-provider (gcp-13)", GithubActions),
                                create_check(
                                    "build-generated-provider (housekeeping (ubuntu-24.04))",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (housekeeping (windows-2022))",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (mini-awsnative)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (nested-module)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (nested-module-thirdparty)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (output-funcs)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (output-funcs-edgeorder)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (plain-object-defaults)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (plain-object-disable-defaults)",
                                    GithubActions,
                                ),
                                create_check("build-generated-provider (random)", GithubActions),
                                create_check(
                                    "build-generated-provider (reserved_names)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (unions-inline)",
                                    GithubActions,
                                ),
                                create_check(
                                    "build-generated-provider (unions-inside-arrays)",
                                    GithubActions,
                                ),
                                create_check("build-housekeeping (ubuntu-24.04)", GithubActions),
                                create_check("build-housekeeping (windows-2022)", GithubActions),
                                create_check("build-native (ubuntu-24.04)", GithubActions),
                                create_check("build-native (windows-2022)", GithubActions),
                                create_check("build-rust-docs", GithubActions),
                                create_check("clippy", GithubActions),
                                create_check(
                                    "Run rust-clippy analyzing (ubuntu-24.04)",
                                    GithubActions,
                                ),
                                create_check(
                                    "Run rust-clippy analyzing (windows-2022)",
                                    GithubActions,
                                ),
                                // GithubAdvancedSecurity
                                create_check("CodeQL", GithubAdvancedSecurity),
                                create_check("clippy", GithubAdvancedSecurity),
                                // Mergify
                                create_check("Mergify Merge Protections", Mergify),
                            ]))
                            .build_struct(),
                    )))
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
        .context(name.to_string())
        .integration_id(Box::new(Some(integration.get_integration_id())))
        .build_struct()
}
