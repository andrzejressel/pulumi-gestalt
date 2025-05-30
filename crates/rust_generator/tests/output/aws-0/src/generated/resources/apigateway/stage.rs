/// Manages an API Gateway Stage. A stage is a named reference to a deployment, which can be done via the `aws.apigateway.Deployment` resource. Stages can be optionally managed further with the `aws.apigateway.BasePathMapping` resource, `aws.apigateway.DomainName` resource, and `aws_api_method_settings` resource. For more information, see the [API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-stages.html).
///
/// ### Managing the API Logging CloudWatch Log Group
///
/// API Gateway provides the ability to [enable CloudWatch API logging](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-logging.html). To manage the CloudWatch Log Group when this feature is enabled, the `aws.cloudwatch.LogGroup` resource can be used where the name matches the API Gateway naming convention. If the CloudWatch Log Group previously exists, import the `aws.cloudwatch.LogGroup` resource into Pulumi as a one time operation. You can recreate the environment without import.
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = rest_api::create("example", RestApiArgs::builder().build_struct());
///     let exampleLogGroup = log_group::create(
///         "exampleLogGroup",
///         LogGroupArgs::builder()
///             .name("API-Gateway-Execution-Logs_${example.id}/${stageName}")
///             .retention_in_days(7)
///             .build_struct(),
///     );
///     let exampleStage = stage::create(
///         "exampleStage",
///         StageArgs::builder().stage_name("${stageName}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_stage` using `REST-API-ID/STAGE-NAME`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/stage:Stage example 12345abcde/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stage {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StageArgs {
        /// Enables access logs for the API stage. See Access Log Settings below.
        #[builder(into, default)]
        pub access_log_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigateway::StageAccessLogSettings>,
        >,
        /// Whether a cache cluster is enabled for the stage
        #[builder(into, default)]
        pub cache_cluster_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Size of the cache cluster for the stage, if enabled. Allowed values include `0.5`, `1.6`, `6.1`, `13.5`, `28.4`, `58.2`, `118` and `237`.
        #[builder(into, default)]
        pub cache_cluster_size: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration settings of a canary deployment. See Canary Settings below.
        #[builder(into, default)]
        pub canary_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigateway::StageCanarySettings>,
        >,
        /// Identifier of a client certificate for the stage.
        #[builder(into, default)]
        pub client_certificate_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the deployment that the stage points to
        #[builder(into)]
        pub deployment: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the stage.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Version of the associated API documentation
        #[builder(into, default)]
        pub documentation_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the stage
        #[builder(into)]
        pub stage_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map that defines the stage variables
        #[builder(into, default)]
        pub variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Whether active tracing with X-ray is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub xray_tracing_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct StageResult {
        /// Enables access logs for the API stage. See Access Log Settings below.
        pub access_log_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigateway::StageAccessLogSettings>,
        >,
        /// ARN
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether a cache cluster is enabled for the stage
        pub cache_cluster_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Size of the cache cluster for the stage, if enabled. Allowed values include `0.5`, `1.6`, `6.1`, `13.5`, `28.4`, `58.2`, `118` and `237`.
        pub cache_cluster_size: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration settings of a canary deployment. See Canary Settings below.
        pub canary_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigateway::StageCanarySettings>,
        >,
        /// Identifier of a client certificate for the stage.
        pub client_certificate_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of the deployment that the stage points to
        pub deployment: pulumi_gestalt_rust::Output<String>,
        /// Description of the stage.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Version of the associated API documentation
        pub documentation_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Execution ARN to be used in `lambda_permission`'s `source_arn`
        /// when allowing API Gateway to invoke a Lambda function,
        /// e.g., `arn:aws:execute-api:eu-west-2:123456789012:z4675bid1j/prod`
        pub execution_arn: pulumi_gestalt_rust::Output<String>,
        /// URL to invoke the API pointing to the stage,
        /// e.g., `https://z4675bid1j.execute-api.eu-west-2.amazonaws.com/prod`
        pub invoke_url: pulumi_gestalt_rust::Output<String>,
        /// ID of the associated REST API
        pub rest_api: pulumi_gestalt_rust::Output<String>,
        /// Name of the stage
        pub stage_name: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Map that defines the stage variables
        pub variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN of the WebAcl associated with the Stage.
        pub web_acl_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether active tracing with X-ray is enabled. Defaults to `false`.
        pub xray_tracing_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StageArgs,
    ) -> StageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_log_settings_binding = args.access_log_settings.get_output(context);
        let cache_cluster_enabled_binding = args
            .cache_cluster_enabled
            .get_output(context);
        let cache_cluster_size_binding = args.cache_cluster_size.get_output(context);
        let canary_settings_binding = args.canary_settings.get_output(context);
        let client_certificate_id_binding = args
            .client_certificate_id
            .get_output(context);
        let deployment_binding = args.deployment.get_output(context);
        let description_binding = args.description.get_output(context);
        let documentation_version_binding = args
            .documentation_version
            .get_output(context);
        let rest_api_binding = args.rest_api.get_output(context);
        let stage_name_binding = args.stage_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let variables_binding = args.variables.get_output(context);
        let xray_tracing_enabled_binding = args.xray_tracing_enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/stage:Stage".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessLogSettings".into(),
                    value: &access_log_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheClusterEnabled".into(),
                    value: &cache_cluster_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheClusterSize".into(),
                    value: &cache_cluster_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "canarySettings".into(),
                    value: &canary_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificateId".into(),
                    value: &client_certificate_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deployment".into(),
                    value: &deployment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "documentationVersion".into(),
                    value: &documentation_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stageName".into(),
                    value: &stage_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "variables".into(),
                    value: &variables_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "xrayTracingEnabled".into(),
                    value: &xray_tracing_enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        StageResult {
            access_log_settings: o.get_field("accessLogSettings"),
            arn: o.get_field("arn"),
            cache_cluster_enabled: o.get_field("cacheClusterEnabled"),
            cache_cluster_size: o.get_field("cacheClusterSize"),
            canary_settings: o.get_field("canarySettings"),
            client_certificate_id: o.get_field("clientCertificateId"),
            deployment: o.get_field("deployment"),
            description: o.get_field("description"),
            documentation_version: o.get_field("documentationVersion"),
            execution_arn: o.get_field("executionArn"),
            invoke_url: o.get_field("invokeUrl"),
            rest_api: o.get_field("restApi"),
            stage_name: o.get_field("stageName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            variables: o.get_field("variables"),
            web_acl_arn: o.get_field("webAclArn"),
            xray_tracing_enabled: o.get_field("xrayTracingEnabled"),
        }
    }
}
