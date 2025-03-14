/// ## Example Usage
///
/// ### Dataform Repository Workflow Config
///
///
/// ```yaml
/// resources:
///   gitRepository:
///     type: gcp:sourcerepo:Repository
///     name: git_repository
///     properties:
///       name: my/repository
///   secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: my_secret
///       replication:
///         auto: {}
///   secretVersion:
///     type: gcp:secretmanager:SecretVersion
///     name: secret_version
///     properties:
///       secret: ${secret.id}
///       secretData: secret-data
///   repository:
///     type: gcp:dataform:Repository
///     properties:
///       name: dataform_repository
///       region: us-central1
///       gitRemoteSettings:
///         url: ${gitRepository.url}
///         defaultBranch: main
///         authenticationTokenSecretVersion: ${secretVersion.id}
///       workspaceCompilationOverrides:
///         defaultDatabase: database
///         schemaSuffix: _suffix
///         tablePrefix: prefix_
///   releaseConfig:
///     type: gcp:dataform:RepositoryReleaseConfig
///     name: release_config
///     properties:
///       project: ${repository.project}
///       region: ${repository.region}
///       repository: ${repository.name}
///       name: my_release
///       gitCommitish: main
///       cronSchedule: 0 7 * * *
///       timeZone: America/New_York
///       codeCompilationConfig:
///         defaultDatabase: gcp-example-project
///         defaultSchema: example-dataset
///         defaultLocation: us-central1
///         assertionSchema: example-assertion-dataset
///         databaseSuffix: ""
///         schemaSuffix: ""
///         tablePrefix: ""
///         vars:
///           var1: value
///   dataformSa:
///     type: gcp:serviceaccount:Account
///     name: dataform_sa
///     properties:
///       accountId: dataform-sa
///       displayName: Dataform Service Account
///   workflow:
///     type: gcp:dataform:RepositoryWorkflowConfig
///     properties:
///       project: ${repository.project}
///       region: ${repository.region}
///       repository: ${repository.name}
///       name: my_workflow
///       releaseConfig: ${releaseConfig.id}
///       invocationConfig:
///         includedTargets:
///           - database: gcp-example-project
///             schema: example-dataset
///             name: target_1
///           - database: gcp-example-project
///             schema: example-dataset
///             name: target_2
///         includedTags:
///           - tag_1
///         transitiveDependenciesIncluded: true
///         transitiveDependentsIncluded: true
///         fullyRefreshIncrementalTablesEnabled: false
///         serviceAccount: ${dataformSa.email}
///       cronSchedule: 0 7 * * *
///       timeZone: America/New_York
/// ```
///
/// ## Import
///
/// RepositoryWorkflowConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/repositories/{{repository}}/workflowConfigs/{{name}}`
///
/// * `{{project}}/{{region}}/{{repository}}/{{name}}`
///
/// * `{{region}}/{{repository}}/{{name}}`
///
/// * `{{repository}}/{{name}}`
///
/// When using the `pulumi import` command, RepositoryWorkflowConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryWorkflowConfig:RepositoryWorkflowConfig default projects/{{project}}/locations/{{region}}/repositories/{{repository}}/workflowConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryWorkflowConfig:RepositoryWorkflowConfig default {{project}}/{{region}}/{{repository}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryWorkflowConfig:RepositoryWorkflowConfig default {{region}}/{{repository}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryWorkflowConfig:RepositoryWorkflowConfig default {{repository}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository_workflow_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryWorkflowConfigArgs {
        /// Optional. Optional schedule (in cron format) for automatic creation of compilation results.
        #[builder(into, default)]
        pub cron_schedule: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. If left unset, a default InvocationConfig will be used.
        /// Structure is documented below.
        #[builder(into, default)]
        pub invocation_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataform::RepositoryWorkflowConfigInvocationConfig,
            >,
        >,
        /// The workflow's name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the region
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the release config whose releaseCompilationResult should be executed. Must be in the format projects/*/locations/*/repositories/*/releaseConfigs/*.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub release_config: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A reference to the Dataform repository
        #[builder(into, default)]
        pub repository: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Specifies the time zone to be used when interpreting cronSchedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC.
        #[builder(into, default)]
        pub time_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RepositoryWorkflowConfigResult {
        /// Optional. Optional schedule (in cron format) for automatic creation of compilation results.
        pub cron_schedule: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. If left unset, a default InvocationConfig will be used.
        /// Structure is documented below.
        pub invocation_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataform::RepositoryWorkflowConfigInvocationConfig,
            >,
        >,
        /// The workflow's name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Records of the 10 most recent scheduled execution attempts, ordered in in descending order of executionTime. Updated whenever automatic creation of a workflow invocation is triggered by cronSchedule.
        /// Structure is documented below.
        pub recent_scheduled_execution_records: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::dataform::RepositoryWorkflowConfigRecentScheduledExecutionRecord,
            >,
        >,
        /// A reference to the region
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the release config whose releaseCompilationResult should be executed. Must be in the format projects/*/locations/*/repositories/*/releaseConfigs/*.
        ///
        ///
        /// - - -
        pub release_config: pulumi_gestalt_rust::Output<String>,
        /// A reference to the Dataform repository
        pub repository: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. Specifies the time zone to be used when interpreting cronSchedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC.
        pub time_zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RepositoryWorkflowConfigArgs,
    ) -> RepositoryWorkflowConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cron_schedule_binding = args.cron_schedule.get_output(context);
        let invocation_config_binding = args.invocation_config.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let release_config_binding = args.release_config.get_output(context);
        let repository_binding = args.repository.get_output(context);
        let time_zone_binding = args.time_zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataform/repositoryWorkflowConfig:RepositoryWorkflowConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cronSchedule".into(),
                    value: &cron_schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "invocationConfig".into(),
                    value: &invocation_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "releaseConfig".into(),
                    value: &release_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repository".into(),
                    value: &repository_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RepositoryWorkflowConfigResult {
            cron_schedule: o.get_field("cronSchedule"),
            invocation_config: o.get_field("invocationConfig"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            recent_scheduled_execution_records: o
                .get_field("recentScheduledExecutionRecords"),
            region: o.get_field("region"),
            release_config: o.get_field("releaseConfig"),
            repository: o.get_field("repository"),
            time_zone: o.get_field("timeZone"),
        }
    }
}
