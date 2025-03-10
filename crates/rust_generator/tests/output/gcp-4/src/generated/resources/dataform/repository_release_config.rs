/// ## Example Usage
///
/// ### Dataform Repository Release Config
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
///   release:
///     type: gcp:dataform:RepositoryReleaseConfig
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
/// ```
///
/// ## Import
///
/// RepositoryReleaseConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/repositories/{{repository}}/releaseConfigs/{{name}}`
///
/// * `{{project}}/{{region}}/{{repository}}/{{name}}`
///
/// * `{{region}}/{{repository}}/{{name}}`
///
/// * `{{repository}}/{{name}}`
///
/// When using the `pulumi import` command, RepositoryReleaseConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryReleaseConfig:RepositoryReleaseConfig default projects/{{project}}/locations/{{region}}/repositories/{{repository}}/releaseConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryReleaseConfig:RepositoryReleaseConfig default {{project}}/{{region}}/{{repository}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryReleaseConfig:RepositoryReleaseConfig default {{region}}/{{repository}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataform/repositoryReleaseConfig:RepositoryReleaseConfig default {{repository}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod repository_release_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RepositoryReleaseConfigArgs {
        /// Optional. If set, fields of codeCompilationConfig override the default compilation settings that are specified in dataform.json.
        /// Structure is documented below.
        #[builder(into, default)]
        pub code_compilation_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataform::RepositoryReleaseConfigCodeCompilationConfig,
            >,
        >,
        /// Optional. Optional schedule (in cron format) for automatic creation of compilation results.
        #[builder(into, default)]
        pub cron_schedule: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Git commit/tag/branch name at which the repository should be compiled. Must exist in the remote repository.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub git_commitish: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The release's name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the region
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the Dataform repository
        #[builder(into, default)]
        pub repository: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Specifies the time zone to be used when interpreting cronSchedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). If left unspecified, the default is UTC.
        #[builder(into, default)]
        pub time_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RepositoryReleaseConfigResult {
        /// Optional. If set, fields of codeCompilationConfig override the default compilation settings that are specified in dataform.json.
        /// Structure is documented below.
        pub code_compilation_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataform::RepositoryReleaseConfigCodeCompilationConfig,
            >,
        >,
        /// Optional. Optional schedule (in cron format) for automatic creation of compilation results.
        pub cron_schedule: pulumi_gestalt_rust::Output<Option<String>>,
        /// Git commit/tag/branch name at which the repository should be compiled. Must exist in the remote repository.
        ///
        ///
        /// - - -
        pub git_commitish: pulumi_gestalt_rust::Output<String>,
        /// The release's name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Records of the 10 most recent scheduled release attempts, ordered in in descending order of releaseTime. Updated whenever automatic creation of a compilation result is triggered by cronSchedule.
        /// Structure is documented below.
        pub recent_scheduled_release_records: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::dataform::RepositoryReleaseConfigRecentScheduledReleaseRecord,
            >,
        >,
        /// A reference to the region
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
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
        args: RepositoryReleaseConfigArgs,
    ) -> RepositoryReleaseConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let code_compilation_config_binding = args
            .code_compilation_config
            .get_output(context);
        let cron_schedule_binding = args.cron_schedule.get_output(context);
        let git_commitish_binding = args.git_commitish.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let repository_binding = args.repository.get_output(context);
        let time_zone_binding = args.time_zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataform/repositoryReleaseConfig:RepositoryReleaseConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "codeCompilationConfig".into(),
                    value: &code_compilation_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cronSchedule".into(),
                    value: &cron_schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gitCommitish".into(),
                    value: &git_commitish_binding.drop_type(),
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
        RepositoryReleaseConfigResult {
            code_compilation_config: o.get_field("codeCompilationConfig"),
            cron_schedule: o.get_field("cronSchedule"),
            git_commitish: o.get_field("gitCommitish"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            recent_scheduled_release_records: o
                .get_field("recentScheduledReleaseRecords"),
            region: o.get_field("region"),
            repository: o.get_field("repository"),
            time_zone: o.get_field("timeZone"),
        }
    }
}
