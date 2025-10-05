/// Provides an AWS Backup plan resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:backup:Plan
///     properties:
///       name: my_example_backup_plan
///       rules:
///         - ruleName: my_example_backup_rule
///           targetVaultName: ${test.name}
///           schedule: cron(0 12 * * ? *)
///           lifecycle:
///             deleteAfter: 14
///       advancedBackupSettings:
///         - backupOptions:
///             WindowsVSS: enabled
///           resourceType: EC2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup Plan using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/plan:Plan test <id>
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlanArgs {
        /// An object that specifies backup options for each resource type.
        #[builder(into, default)]
        pub advanced_backup_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::backup::PlanAdvancedBackupSetting>>,
        >,
        /// The display name of a backup plan.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A rule object that specifies a scheduled task that is used to back up a selection of resources.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::backup::PlanRule>,
        >,
        /// Metadata that you can assign to help organize the plans you create. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PlanResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An object that specifies backup options for each resource type.
        pub advanced_backup_settings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::backup::PlanAdvancedBackupSetting>>,
        >,
        /// The ARN of the backup plan.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The display name of a backup plan.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A rule object that specifies a scheduled task that is used to back up a selection of resources.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::backup::PlanRule>,
        >,
        /// Metadata that you can assign to help organize the plans you create. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Unique, randomly generated, Unicode, UTF-8 encoded string that serves as the version ID of the backup plan.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PlanArgs,
    ) -> PlanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let advanced_backup_settings_binding = args
            .advanced_backup_settings
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:backup/plan:Plan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "advancedBackupSettings".into(),
                    value: &advanced_backup_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PlanResult {
            id: o.get_field("id"),
            advanced_backup_settings: o.get_field("advancedBackupSettings"),
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            rules: o.get_field("rules"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version: o.get_field("version"),
        }
    }
}
