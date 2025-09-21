use crate::github::functions::get_repository::GetRepositoryArgs;
use crate::github::repository_ruleset::RepositoryRulesetArgs;
use crate::github::types::{
    RepositoryRulesetConditions, RepositoryRulesetConditionsRefName, RepositoryRulesetRules,
    RepositoryRulesetRulesRequiredStatusChecksRequiredCheck,
};
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
            GithubIntegration::Any => 0,
            GithubIntegration::GithubActions => 15368,
            GithubIntegration::GithubAdvancedSecurity => 57789,
            GithubIntegration::Mergify => 10562,
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
                    .required_status_checks(Box::new(Some(
                        RepositoryRulesetRulesRequiredStatusChecks::builder()
                            .strict_required_status_checks_policy(Box::new(Some(false)))
                            .required_checks(Box::new(vec![
                                create_check("build-generated-provider (functions-secrets)", 15368),
                                create_check("build-generated-provider (azure-12)", 15368),
                                create_check("build-generated-provider (gcp-1)", 15368),
                                create_check("Analyze C", 15368),
                                create_check("build-c (ubuntu-24.04)", 15368),
                                create_check("build-generated-provider (azure-7)", 15368),
                                create_check("build-housekeeping (windows-2022)", 15368),
                                create_check("build-base (ubuntu-24.04)", 15368),
                                create_check("build-generated-provider (aws-3)", 15368),
                                create_check("build-generated-provider (gcp-11)", 15368),
                                create_check("build-generated-provider (aws-9)", 15368),
                                create_check("build-generated-provider (aws-16)", 15368),
                                create_check("build-generated-provider (aws-0)", 15368),
                                create_check("build-generated-provider (aws-21)", 15368),
                                create_check("build-generated-provider (aws-4)", 15368),
                                create_check("build-generated-provider (azure-11)", 15368),
                                create_check("build-generated-provider (filtering-0)", 15368),
                                create_check("build-generated-provider (aws-11)", 15368),
                                create_check("build-generated-provider (gcp-6)", 15368),
                                create_check("build-generated-provider (azure-0)", 15368),
                                create_check("build-generated-provider (aws-14)", 15368),
                                create_check("Run rust-clippy analyzing (windows-2022)", 15368),
                                create_check("build-examples (ubuntu-24.04)", 15368),
                                create_check("build-generated-provider (gcp-12)", 15368),
                                create_check("build-generated-provider (aws-1)", 15368),
                                create_check("build-generated-provider (docker)", 15368),
                                create_check("build-generated-provider (gcp-13)", 15368),
                                create_check("build-rust-docs", 15368),
                                create_check("build-generated-provider (gcp-9)", 15368),
                                create_check("build-housekeeping (ubuntu-24.04)", 15368),
                                create_check("build-generated-provider (gcp-3)", 15368),
                                create_check("build-generated-provider (unions-inline)", 15368),
                                create_check(
                                    "build-generated-provider (nested-module-thirdparty)",
                                    15368,
                                ),
                                create_check("build-generated-provider (azure-5)", 15368),
                                create_check(
                                    "build-generated-provider (plain-object-defaults)",
                                    15368,
                                ),
                                create_check("build-native (ubuntu-24.04)", 15368),
                                create_check("build-generated-provider (azure-8)", 15368),
                                create_check("build-generated-provider (gcp-4)", 15368),
                                create_check("build-generated-provider (azure-2)", 15368),
                                create_check("build-generated-provider (aws-13)", 15368),
                                create_check("build-generated-provider (cyclic-types)", 15368),
                                create_check("Run rust-clippy analyzing (ubuntu-24.04)", 15368),
                                create_check(
                                    "build-generated-provider (output-funcs-edgeorder)",
                                    15368,
                                ),
                                create_check("build-generated-provider (aws-19)", 15368),
                                create_check("build-generated-provider (aws-6)", 15368),
                                create_check("build-examples (windows-2022)", 15368),
                                create_check("build-generated-provider (filtering-2)", 15368),
                                create_check("build-generated-provider (azure-13)", 15368),
                                create_check("build-base (windows-2022)", 15368),
                                create_check("build-generated-provider (gcp-0)", 15368),
                                create_check("build-generated-provider (gcp-8)", 15368),
                                create_check("build-generated-provider (azure-6)", 15368),
                                create_check("CodeQL", 57789),
                                create_check(
                                    "build-generated-provider (unions-inside-arrays)",
                                    15368,
                                ),
                                create_check("build-generated-provider (aws-2)", 15368),
                                create_check("build-c (windows-2022)", 15368),
                                create_check("build-generated-provider (gcp-10)", 15368),
                                create_check("build-generated-provider (aws-17)", 15368),
                                create_check("build-generated-provider (aws-8)", 15368),
                                create_check("build-generated-provider (aws-20)", 15368),
                                create_check(
                                    "build-generated-provider (plain-object-disable-defaults)",
                                    15368,
                                ),
                                create_check("build-generated-provider (aws-5)", 15368),
                                create_check("build-generated-provider (azure-10)", 15368),
                                create_check("build-generated-provider (filtering-1)", 15368),
                                create_check("build-generated-provider (mini-awsnative)", 15368),
                                create_check("build-generated-provider (different-enum)", 15368),
                                create_check("build-generated-provider (aws-10)", 15368),
                                create_check("clippy", 57789),
                                create_check("docs/readthedocs.org:pulumi-gestalt", 0),
                                create_check("build-generated-provider (gcp-7)", 15368),
                                create_check("build-generated-provider (workarounds)", 15368),
                                create_check("build-generated-provider (azure-1)", 15368),
                                create_check("build-generated-provider (gcp-2)", 15368),
                                create_check("build-generated-provider (random)", 15368),
                                create_check("Mergify Merge Protections", 10562),
                                create_check("build-generated-provider (array-of-enum-map)", 15368),
                                create_check("build-generated-provider (nested-module)", 15368),
                                create_check("build-generated-provider (azure-4)", 15368),
                                create_check("build-generated-provider (aws-15)", 15368),
                                create_check("build-generated-provider (reserved_names)", 15368),
                                create_check("build-generated-provider (azure-9)", 15368),
                                create_check(
                                    "build-generated-provider (azure-native-nested-types)",
                                    15368,
                                ),
                                create_check("Analyze Go", 15368),
                                create_check("build-generated-provider (gcp-5)", 15368),
                                create_check("build-generated-provider (azure-3)", 15368),
                                create_check("build-generated-provider (aws-12)", 15368),
                                create_check("build-native (windows-2022)", 15368),
                                create_check("build-generated-provider (cloudflare)", 15368),
                                create_check("build-generated-provider (aws-7)", 15368),
                                create_check("build-generated-provider (aws-18)", 15368),
                                create_check("build-generated-provider (output-funcs)", 15368),
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
    integration_id: i32,
) -> RepositoryRulesetRulesRequiredStatusChecksRequiredCheck {
    RepositoryRulesetRulesRequiredStatusChecksRequiredCheck::builder()
        .context(name.to_string())
        .integration_id(Box::new(Some(integration_id)))
        .build_struct()
}
