/// Manages an Amazon Managed Streaming for Kafka configuration. More information can be found on the [MSK Developer Guide](https://docs.aws.amazon.com/msk/latest/developerguide/msk-configuration.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = configuration::create(
///         "example",
///         ConfigurationArgs::builder()
///             .kafka_versions(vec!["2.1.0",])
///             .name("example")
///             .server_properties(
///                 "auto.create.topics.enable = true\ndelete.topic.enable = true",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MSK configurations using the configuration ARN. For example:
///
/// ```sh
/// $ pulumi import aws:msk/configuration:Configuration example arn:aws:kafka:us-west-2:123456789012:configuration/example/279c0212-d057-4dba-9aa9-1c4e5a25bfc7-3
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationArgs {
        /// Description of the configuration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// List of Apache Kafka versions which can use this configuration.
        #[builder(into, default)]
        pub kafka_versions: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
        /// Name of the configuration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// Contents of the server.properties file. Supported properties are documented in the [MSK Developer Guide](https://docs.aws.amazon.com/msk/latest/developerguide/msk-configuration-properties.html).
        #[builder(into)]
        pub server_properties: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the configuration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of Apache Kafka versions which can use this configuration.
        pub kafka_versions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Latest revision of the configuration.
        pub latest_revision: pulumi_gestalt_rust::Output<i32>,
        /// Name of the configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Contents of the server.properties file. Supported properties are documented in the [MSK Developer Guide](https://docs.aws.amazon.com/msk/latest/developerguide/msk-configuration-properties.html).
        pub server_properties: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigurationArgs,
    ) -> ConfigurationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigurationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ConfigurationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigurationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ConfigurationResult {
        let description_binding = args.description.get_output(ctx);
        let kafka_versions_binding = args.kafka_versions.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let server_properties_binding = args.server_properties.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:msk/configuration:Configuration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kafkaVersions".into(),
                    value: &kafka_versions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverProperties".into(),
                    value: &server_properties_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ConfigurationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            kafka_versions: o.get_field("kafkaVersions"),
            latest_revision: o.get_field("latestRevision"),
            name: o.get_field("name"),
            server_properties: o.get_field("serverProperties"),
        }
    }
}
