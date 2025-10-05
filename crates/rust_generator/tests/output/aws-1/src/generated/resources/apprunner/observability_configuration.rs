/// Manages an App Runner Observability Configuration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apprunner:ObservabilityConfiguration
///     properties:
///       observabilityConfigurationName: example
///       traceConfiguration:
///         vendor: AWSXRAY
///       tags:
///         Name: example-apprunner-observability-configuration
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Runner Observability Configuration using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:apprunner/observabilityConfiguration:ObservabilityConfiguration example arn:aws:apprunner:us-east-1:1234567890:observabilityconfiguration/example/1/d75bc7ea55b71e724fe5c23452fe22a1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod observability_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObservabilityConfigurationArgs {
        /// Name of the observability configuration.
        #[builder(into)]
        pub observability_configuration_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration of the tracing feature within this observability configuration. If you don't specify it, App Runner doesn't enable tracing. See Trace Configuration below for more details.
        #[builder(into, default)]
        pub trace_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::apprunner::ObservabilityConfigurationTraceConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ObservabilityConfigurationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of this observability configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether the observability configuration has the highest `observability_configuration_revision` among all configurations that share the same `observability_configuration_name`.
        pub latest: pulumi_gestalt_rust::Output<bool>,
        /// Name of the observability configuration.
        pub observability_configuration_name: pulumi_gestalt_rust::Output<String>,
        /// The revision of this observability configuration.
        pub observability_configuration_revision: pulumi_gestalt_rust::Output<i32>,
        /// Current state of the observability configuration. An INACTIVE configuration revision has been deleted and can't be used. It is permanently removed some time after deletion.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration of the tracing feature within this observability configuration. If you don't specify it, App Runner doesn't enable tracing. See Trace Configuration below for more details.
        pub trace_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::apprunner::ObservabilityConfigurationTraceConfiguration,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ObservabilityConfigurationArgs,
    ) -> ObservabilityConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let observability_configuration_name_binding = args
            .observability_configuration_name
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let trace_configuration_binding = args.trace_configuration.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apprunner/observabilityConfiguration:ObservabilityConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "observabilityConfigurationName".into(),
                    value: &observability_configuration_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "traceConfiguration".into(),
                    value: &trace_configuration_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ObservabilityConfigurationResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            latest: o.get_field("latest"),
            observability_configuration_name: o
                .get_field("observabilityConfigurationName"),
            observability_configuration_revision: o
                .get_field("observabilityConfigurationRevision"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            trace_configuration: o.get_field("traceConfiguration"),
        }
    }
}
