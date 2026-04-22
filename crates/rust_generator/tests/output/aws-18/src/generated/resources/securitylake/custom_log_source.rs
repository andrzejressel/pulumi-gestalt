/// Resource for managing an AWS Security Lake Custom Log Source.
///
/// > **NOTE:** The underlying `aws.securitylake.DataLake` must be configured before creating the `aws.securitylake.CustomLogSource`. Use a `depends_on` statement.
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
///     let example = custom_log_source::create(
///         "example",
///         CustomLogSourceArgs::builder()
///             .configuration(
///                 CustomLogSourceConfiguration::builder()
///                     .crawlerConfiguration(
///                         CustomLogSourceConfigurationCrawlerConfiguration::builder()
///                             .roleArn("${customLog.arn}")
///                             .build_struct(),
///                     )
///                     .providerIdentity(
///                         CustomLogSourceConfigurationProviderIdentity::builder()
///                             .externalId("example-id")
///                             .principal("123456789012")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .event_classes(vec!["FILE_ACTIVITY",])
///             .source_name("example-name")
///             .source_version("1.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Custom log sources using the source name. For example:
///
/// ```sh
/// $ pulumi import aws:securitylake/customLogSource:CustomLogSource example example-name
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod custom_log_source {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomLogSourceArgs {
        /// The configuration for the third-party custom source.
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::Input<
            Option<super::super::types::securitylake::CustomLogSourceConfiguration>,
        >,
        /// The Open Cybersecurity Schema Framework (OCSF) event classes which describes the type of data that the custom source will send to Security Lake.
        #[builder(into, default)]
        pub event_classes: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
        /// Specify the name for a third-party custom source.
        /// This must be a Regionally unique value.
        /// Has a maximum length of 20.
        #[builder(into)]
        pub source_name: pulumi_gestalt_rust::Input<String>,
        /// Specify the source version for the third-party custom source, to limit log collection to a specific version of custom data source.
        #[builder(into, default)]
        pub source_version: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CustomLogSourceResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The attributes of a third-party custom source.
        pub attributes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::securitylake::CustomLogSourceAttribute>,
        >,
        /// The configuration for the third-party custom source.
        pub configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::securitylake::CustomLogSourceConfiguration>,
        >,
        /// The Open Cybersecurity Schema Framework (OCSF) event classes which describes the type of data that the custom source will send to Security Lake.
        pub event_classes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The details of the log provider for a third-party custom source.
        pub provider_details: pulumi_gestalt_rust::Output<
            Vec<super::super::types::securitylake::CustomLogSourceProviderDetail>,
        >,
        /// Specify the name for a third-party custom source.
        /// This must be a Regionally unique value.
        /// Has a maximum length of 20.
        pub source_name: pulumi_gestalt_rust::Output<String>,
        /// Specify the source version for the third-party custom source, to limit log collection to a specific version of custom data source.
        pub source_version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomLogSourceArgs,
    ) -> CustomLogSourceResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomLogSourceArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> CustomLogSourceResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomLogSourceArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> CustomLogSourceResult {
        let configuration_binding = args.configuration.get_output(ctx);
        let event_classes_binding = args.event_classes.get_output(ctx);
        let source_name_binding = args.source_name.get_output(ctx);
        let source_version_binding = args.source_version.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:securitylake/customLogSource:CustomLogSource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventClasses".into(),
                    value: &event_classes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceName".into(),
                    value: &source_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceVersion".into(),
                    value: &source_version_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        CustomLogSourceResult {
            id: o.get_id(),
            urn: o.get_urn(),
            attributes: o.get_field("attributes"),
            configuration: o.get_field("configuration"),
            event_classes: o.get_field("eventClasses"),
            provider_details: o.get_field("providerDetails"),
            source_name: o.get_field("sourceName"),
            source_version: o.get_field("sourceVersion"),
        }
    }
}
