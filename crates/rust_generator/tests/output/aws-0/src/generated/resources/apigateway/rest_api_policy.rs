/// Provides an API Gateway REST API Policy.
///
/// > **Note:** Amazon API Gateway Version 1 resources are used for creating and deploying REST APIs. To create and deploy WebSocket and HTTP APIs, use Amazon API Gateway Version 2 resources.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   testRestApi:
///     type: aws:apigateway:RestApi
///     name: test
///     properties:
///       name: example-rest-api
///   testRestApiPolicy:
///     type: aws:apigateway:RestApiPolicy
///     name: test
///     properties:
///       restApiId: ${testRestApi.id}
///       policy: ${test.json}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '*'
///             actions:
///               - execute-api:Invoke
///             resources:
///               - ${testRestApi.executionArn}
///             conditions:
///               - test: IpAddress
///                 variable: aws:SourceIp
///                 values:
///                   - 123.123.123.123/32
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_rest_api_policy` using the REST API ID. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/restApiPolicy:RestApiPolicy example 12345abcde
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod rest_api_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RestApiPolicyArgs {
        /// JSON formatted policy document that controls access to the API Gateway.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::Input<String>,
        /// ID of the REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct RestApiPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// JSON formatted policy document that controls access to the API Gateway.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// ID of the REST API.
        pub rest_api_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RestApiPolicyArgs,
    ) -> RestApiPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RestApiPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> RestApiPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RestApiPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> RestApiPolicyResult {
        let policy_binding = args.policy.get_output(ctx);
        let rest_api_id_binding = args.rest_api_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/restApiPolicy:RestApiPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: &policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApiId".into(),
                    value: &rest_api_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        RestApiPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            policy: o.get_field("policy"),
            rest_api_id: o.get_field("restApiId"),
        }
    }
}
