/// Resource for managing a QuickSight Dashboard.
///
/// ## Example Usage
///
/// ### From Source Template
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = dashboard::create(
///         "example",
///         DashboardArgs::builder()
///             .dashboard_id("example-id")
///             .name("example-name")
///             .source_entity(
///                 DashboardSourceEntity::builder()
///                     .sourceTemplate(
///                         DashboardSourceEntitySourceTemplate::builder()
///                             .arn("${source.arn}")
///                             .dataSetReferences(
///                                 vec![
///                                     DashboardSourceEntitySourceTemplateDataSetReference::builder()
///                                     .dataSetArn("${dataset.arn}").dataSetPlaceholder("1")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .version_description("version")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Definition
///
/// ```yaml
/// resources:
///   example:
///     type: aws:quicksight:Dashboard
///     properties:
///       dashboardId: example-id
///       name: example-name
///       versionDescription: version
///       definition:
///         dataSetIdentifiersDeclarations:
///           - dataSetArn: ${dataset.arn}
///             identifier: '1'
///         sheets:
///           - title: Example
///             sheetId: Example1
///             visuals:
///               - lineChartVisual:
///                   visualId: LineChart
///                   title:
///                     formatText:
///                       plainText: Line Chart Example
///                   chartConfiguration:
///                     fieldWells:
///                       lineChartAggregatedFieldWells:
///                         categories:
///                           - categoricalDimensionField:
///                               fieldId: '1'
///                               column:
///                                 dataSetIdentifier: '1'
///                                 columnName: Column1
///                         values:
///                           - categoricalMeasureField:
///                               fieldId: '2'
///                               column:
///                                 dataSetIdentifier: '1'
///                                 columnName: Column1
///                               aggregationFunction: COUNT
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a QuickSight Dashboard using the AWS account ID and dashboard ID separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/dashboard:Dashboard example 123456789012,example-id
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod dashboard {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DashboardArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::Input<Option<String>>,
        /// Identifier for the dashboard.
        #[builder(into)]
        pub dashboard_id: pulumi_gestalt_rust::Input<String>,
        /// Options for publishing the dashboard. See dashboard_publish_options.
        #[builder(into, default)]
        pub dashboard_publish_options: pulumi_gestalt_rust::Input<
            Option<super::super::types::quicksight::DashboardDashboardPublishOptions>,
        >,
        /// Display name for the dashboard.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The parameters for the creation of the dashboard, which you want to use to override the default settings. A dashboard can have any type of parameters, and some parameters might accept multiple values. See parameters.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::Input<
            Option<super::super::types::quicksight::DashboardParameters>,
        >,
        /// A set of resource permissions on the dashboard. Maximum of 64 items. See permissions.
        #[builder(into, default)]
        pub permissions: pulumi_gestalt_rust::Input<
            Option<Vec<super::super::types::quicksight::DashboardPermission>>,
        >,
        /// The entity that you are using as a source when you create the dashboard (template). Only one of `definition` or `source_entity` should be configured. See source_entity.
        #[builder(into, default)]
        pub source_entity: pulumi_gestalt_rust::Input<
            Option<super::super::types::quicksight::DashboardSourceEntity>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. The theme ARN must exist in the same AWS account where you create the dashboard.
        #[builder(into, default)]
        pub theme_arn: pulumi_gestalt_rust::Input<Option<String>>,
        /// A description of the current dashboard version being created/updated.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub version_description: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct DashboardResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the dashboard.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The time that the dashboard was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Identifier for the dashboard.
        pub dashboard_id: pulumi_gestalt_rust::Output<String>,
        /// Options for publishing the dashboard. See dashboard_publish_options.
        pub dashboard_publish_options: pulumi_gestalt_rust::Output<
            super::super::types::quicksight::DashboardDashboardPublishOptions,
        >,
        pub last_published_time: pulumi_gestalt_rust::Output<String>,
        /// The time that the dashboard was last updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// Display name for the dashboard.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parameters for the creation of the dashboard, which you want to use to override the default settings. A dashboard can have any type of parameters, and some parameters might accept multiple values. See parameters.
        pub parameters: pulumi_gestalt_rust::Output<
            super::super::types::quicksight::DashboardParameters,
        >,
        /// A set of resource permissions on the dashboard. Maximum of 64 items. See permissions.
        pub permissions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::quicksight::DashboardPermission>>,
        >,
        /// The entity that you are using as a source when you create the dashboard (template). Only one of `definition` or `source_entity` should be configured. See source_entity.
        pub source_entity: pulumi_gestalt_rust::Output<
            Option<super::super::types::quicksight::DashboardSourceEntity>,
        >,
        /// Amazon Resource Name (ARN) of a template that was used to create this dashboard.
        pub source_entity_arn: pulumi_gestalt_rust::Output<String>,
        /// The dashboard creation status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. The theme ARN must exist in the same AWS account where you create the dashboard.
        pub theme_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// A description of the current dashboard version being created/updated.
        ///
        /// The following arguments are optional:
        pub version_description: pulumi_gestalt_rust::Output<String>,
        /// The version number of the dashboard version.
        pub version_number: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DashboardArgs,
    ) -> DashboardResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DashboardArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DashboardResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DashboardArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DashboardResult {
        let aws_account_id_binding = args.aws_account_id.get_output(ctx);
        let dashboard_id_binding = args.dashboard_id.get_output(ctx);
        let dashboard_publish_options_binding = args
            .dashboard_publish_options
            .get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let parameters_binding = args.parameters.get_output(ctx);
        let permissions_binding = args.permissions.get_output(ctx);
        let source_entity_binding = args.source_entity.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let theme_arn_binding = args.theme_arn.get_output(ctx);
        let version_description_binding = args.version_description.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:quicksight/dashboard:Dashboard".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dashboardId".into(),
                    value: &dashboard_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dashboardPublishOptions".into(),
                    value: &dashboard_publish_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceEntity".into(),
                    value: &source_entity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "themeArn".into(),
                    value: &theme_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionDescription".into(),
                    value: &version_description_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DashboardResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            created_time: o.get_field("createdTime"),
            dashboard_id: o.get_field("dashboardId"),
            dashboard_publish_options: o.get_field("dashboardPublishOptions"),
            last_published_time: o.get_field("lastPublishedTime"),
            last_updated_time: o.get_field("lastUpdatedTime"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            permissions: o.get_field("permissions"),
            source_entity: o.get_field("sourceEntity"),
            source_entity_arn: o.get_field("sourceEntityArn"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            theme_arn: o.get_field("themeArn"),
            version_description: o.get_field("versionDescription"),
            version_number: o.get_field("versionNumber"),
        }
    }
}
