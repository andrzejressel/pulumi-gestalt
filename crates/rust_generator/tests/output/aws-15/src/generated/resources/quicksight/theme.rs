/// Resource for managing a QuickSight Theme.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = theme::create(
///         "example",
///         ThemeArgs::builder()
///             .base_theme_id("MIDNIGHT")
///             .configuration(
///                 ThemeConfiguration::builder()
///                     .dataColorPalette(
///                         ThemeConfigurationDataColorPalette::builder()
///                             .colors(
///                                 vec![
///                                     "#FFFFFF", "#111111", "#222222", "#333333", "#444444",
///                                     "#555555", "#666666", "#777777", "#888888", "#999999",
///                                 ],
///                             )
///                             .emptyFillColor("#FFFFFF")
///                             .minMaxGradients(vec!["#FFFFFF", "#111111",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .theme_id("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a QuickSight Theme using the AWS account ID and theme ID separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/theme:Theme example 123456789012,example-id
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod theme {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThemeArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ID of the theme that a custom theme will inherit from. All themes inherit from one of the starting themes defined by Amazon QuickSight. For a list of the starting themes, use ListThemes or choose Themes from within an analysis.
        #[builder(into)]
        pub base_theme_id: pulumi_gestalt_rust::Input<String>,
        /// The theme configuration, which contains the theme display properties. See configuration.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::Input<
            Option<super::super::types::quicksight::ThemeConfiguration>,
        >,
        /// Display name of the theme.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// A set of resource permissions on the theme. Maximum of 64 items. See permissions.
        #[builder(into, default)]
        pub permissions: pulumi_gestalt_rust::Input<
            Option<Vec<super::super::types::quicksight::ThemePermission>>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the theme.
        #[builder(into)]
        pub theme_id: pulumi_gestalt_rust::Input<String>,
        /// A description of the current theme version being created/updated.
        #[builder(into, default)]
        pub version_description: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ThemeResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the theme.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the theme that a custom theme will inherit from. All themes inherit from one of the starting themes defined by Amazon QuickSight. For a list of the starting themes, use ListThemes or choose Themes from within an analysis.
        pub base_theme_id: pulumi_gestalt_rust::Output<String>,
        /// The theme configuration, which contains the theme display properties. See configuration.
        ///
        /// The following arguments are optional:
        pub configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::quicksight::ThemeConfiguration>,
        >,
        /// The time that the theme was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// The time that the theme was last updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// Display name of the theme.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A set of resource permissions on the theme. Maximum of 64 items. See permissions.
        pub permissions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::quicksight::ThemePermission>>,
        >,
        /// The theme creation status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifier of the theme.
        pub theme_id: pulumi_gestalt_rust::Output<String>,
        /// A description of the current theme version being created/updated.
        pub version_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The version number of the theme version.
        pub version_number: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ThemeArgs,
    ) -> ThemeResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ThemeArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ThemeResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ThemeArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ThemeResult {
        let aws_account_id_binding = args.aws_account_id.get_output(ctx);
        let base_theme_id_binding = args.base_theme_id.get_output(ctx);
        let configuration_binding = args.configuration.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let permissions_binding = args.permissions.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let theme_id_binding = args.theme_id.get_output(ctx);
        let version_description_binding = args.version_description.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:quicksight/theme:Theme".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baseThemeId".into(),
                    value: &base_theme_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "themeId".into(),
                    value: &theme_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionDescription".into(),
                    value: &version_description_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ThemeResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            base_theme_id: o.get_field("baseThemeId"),
            configuration: o.get_field("configuration"),
            created_time: o.get_field("createdTime"),
            last_updated_time: o.get_field("lastUpdatedTime"),
            name: o.get_field("name"),
            permissions: o.get_field("permissions"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            theme_id: o.get_field("themeId"),
            version_description: o.get_field("versionDescription"),
            version_number: o.get_field("versionNumber"),
        }
    }
}
