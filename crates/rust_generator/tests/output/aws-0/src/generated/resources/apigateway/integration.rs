/// Provides an HTTP Method Integration for an API Gateway Integration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   myDemoAPI:
///     type: aws:apigateway:RestApi
///     name: MyDemoAPI
///     properties:
///       name: MyDemoAPI
///       description: This is my API for demonstration purposes
///   myDemoResource:
///     type: aws:apigateway:Resource
///     name: MyDemoResource
///     properties:
///       restApi: ${myDemoAPI.id}
///       parentId: ${myDemoAPI.rootResourceId}
///       pathPart: mydemoresource
///   myDemoMethod:
///     type: aws:apigateway:Method
///     name: MyDemoMethod
///     properties:
///       restApi: ${myDemoAPI.id}
///       resourceId: ${myDemoResource.id}
///       httpMethod: GET
///       authorization: NONE
///   myDemoIntegration:
///     type: aws:apigateway:Integration
///     name: MyDemoIntegration
///     properties:
///       restApi: ${myDemoAPI.id}
///       resourceId: ${myDemoResource.id}
///       httpMethod: ${myDemoMethod.httpMethod}
///       type: MOCK
///       cacheKeyParameters:
///         - method.request.path.param
///       cacheNamespace: foobar
///       timeoutMilliseconds: 29000
///       requestParameters:
///         integration.request.header.X-Authorization: '''static'''
///       requestTemplates:
///         application/xml: |
///           {
///              "body" : $input.json('$')
///           }
/// ```
///
/// ## Lambda integration
///
/// ```yaml
/// configuration:
///   # Variables
///   myregion:
///     type: dynamic
///   accountId:
///     type: dynamic
/// resources:
///   # API Gateway
///   api:
///     type: aws:apigateway:RestApi
///     properties:
///       name: myapi
///   resource:
///     type: aws:apigateway:Resource
///     properties:
///       pathPart: resource
///       parentId: ${api.rootResourceId}
///       restApi: ${api.id}
///   method:
///     type: aws:apigateway:Method
///     properties:
///       restApi: ${api.id}
///       resourceId: ${resource.id}
///       httpMethod: GET
///       authorization: NONE
///   integration:
///     type: aws:apigateway:Integration
///     properties:
///       restApi: ${api.id}
///       resourceId: ${resource.id}
///       httpMethod: ${method.httpMethod}
///       integrationHttpMethod: POST
///       type: AWS_PROXY
///       uri: ${lambda.invokeArn}
///   # Lambda
///   apigwLambda:
///     type: aws:lambda:Permission
///     name: apigw_lambda
///     properties:
///       statementId: AllowExecutionFromAPIGateway
///       action: lambda:InvokeFunction
///       function: ${lambda.name}
///       principal: apigateway.amazonaws.com
///       sourceArn: arn:aws:execute-api:${myregion}:${accountId}:${api.id}/*/${method.httpMethod}${resource.path}
///   lambda:
///     type: aws:lambda:Function
///     properties:
///       code:
///         fn::FileArchive: lambda.zip
///       name: mylambda
///       role: ${role.arn}
///       handler: lambda.lambda_handler
///       runtime: python3.12
///       sourceCodeHash:
///         fn::invoke:
///           function: std:filebase64sha256
///           arguments:
///             input: lambda.zip
///           return: result
///   role:
///     type: aws:iam:Role
///     properties:
///       name: myrole
///       assumeRolePolicy: ${assumeRole.json}
/// variables:
///   # IAM
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - lambda.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ## VPC Link
///
/// ```yaml
/// configuration:
///   name:
///     type: dynamic
///   subnetId:
///     type: dynamic
/// resources:
///   test:
///     type: aws:lb:LoadBalancer
///     properties:
///       name: ${name}
///       internal: true
///       loadBalancerType: network
///       subnets:
///         - ${subnetId}
///   testVpcLink:
///     type: aws:apigateway:VpcLink
///     name: test
///     properties:
///       name: ${name}
///       targetArn: ${test.arn}
///   testRestApi:
///     type: aws:apigateway:RestApi
///     name: test
///     properties:
///       name: ${name}
///   testResource:
///     type: aws:apigateway:Resource
///     name: test
///     properties:
///       restApi: ${testRestApi.id}
///       parentId: ${testRestApi.rootResourceId}
///       pathPart: test
///   testMethod:
///     type: aws:apigateway:Method
///     name: test
///     properties:
///       restApi: ${testRestApi.id}
///       resourceId: ${testResource.id}
///       httpMethod: GET
///       authorization: NONE
///       requestModels:
///         application/json: Error
///   testIntegration:
///     type: aws:apigateway:Integration
///     name: test
///     properties:
///       restApi: ${testRestApi.id}
///       resourceId: ${testResource.id}
///       httpMethod: ${testMethod.httpMethod}
///       requestTemplates:
///         application/json: ""
///         application/xml: |-
///           #set($inputRoot = $input.path('$'))
///           { }
///       requestParameters:
///         integration.request.header.X-Authorization: '''static'''
///         integration.request.header.X-Foo: '''Bar'''
///       type: HTTP
///       uri: https://www.google.de
///       integrationHttpMethod: GET
///       passthroughBehavior: WHEN_NO_MATCH
///       contentHandling: CONVERT_TO_TEXT
///       connectionType: VPC_LINK
///       connectionId: ${testVpcLink.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_integration` using `REST-API-ID/RESOURCE-ID/HTTP-METHOD`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/integration:Integration example 12345abcde/67890fghij/GET
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationArgs {
        /// List of cache key parameters for the integration.
        #[builder(into, default)]
        pub cache_key_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Integration's cache namespace.
        #[builder(into, default)]
        pub cache_namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the VpcLink used for the integration. **Required** if `connection_type` is `VPC_LINK`
        #[builder(into, default)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Integration input's [connectionType](https://docs.aws.amazon.com/apigateway/api-reference/resource/integration/#connectionType). Valid values are `INTERNET` (default for connections through the public routable internet), and `VPC_LINK` (for private connections between API Gateway and a network load balancer in a VPC).
        #[builder(into, default)]
        pub connection_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// How to handle request payload content type conversions. Supported values are `CONVERT_TO_BINARY` and `CONVERT_TO_TEXT`. If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the passthroughBehaviors is configured to support payload pass-through.
        #[builder(into, default)]
        pub content_handling: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Credentials required for the integration. For `AWS` integrations, 2 options are available. To specify an IAM Role for Amazon API Gateway to assume, use the role's ARN. To require that the caller's identity be passed through from the request, specify the string `arn:aws:iam::\*:user/\*`.
        #[builder(into, default)]
        pub credentials: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// HTTP method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTION`, `ANY`)
        /// when calling the associated resource.
        #[builder(into)]
        pub http_method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Integration HTTP method
        /// (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONs`, `ANY`, `PATCH`) specifying how API Gateway will interact with the back end.
        /// **Required** if `type` is `AWS`, `AWS_PROXY`, `HTTP` or `HTTP_PROXY`.
        /// Not all methods are compatible with all `AWS` integrations.
        /// e.g., Lambda function [can only be invoked](https://github.com/awslabs/aws-apigateway-importer/issues/9#issuecomment-129651005) via `POST`.
        #[builder(into, default)]
        pub integration_http_method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Integration passthrough behavior (`WHEN_NO_MATCH`, `WHEN_NO_TEMPLATES`, `NEVER`).  **Required** if `request_templates` is used.
        #[builder(into, default)]
        pub passthrough_behavior: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of request query string parameters and headers that should be passed to the backend responder.
        /// For example: `request_parameters = { "integration.request.header.X-Some-Other-Header" = "method.request.header.X-Some-Header" }`
        #[builder(into, default)]
        pub request_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of the integration's request templates.
        #[builder(into, default)]
        pub request_templates: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// API resource ID.
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the associated REST API.
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Custom timeout between 50 and 300,000 milliseconds. The default value is 29,000 milliseconds. You need to raise a [Service Quota Ticket](https://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html) to increase time beyond 29,000 milliseconds.
        #[builder(into, default)]
        pub timeout_milliseconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// TLS configuration. See below.
        #[builder(into, default)]
        pub tls_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigateway::IntegrationTlsConfig>,
        >,
        /// Integration input's [type](https://docs.aws.amazon.com/apigateway/api-reference/resource/integration/). Valid values are `HTTP` (for HTTP backends), `MOCK` (not calling any real backend), `AWS` (for AWS services), `AWS_PROXY` (for Lambda proxy integration) and `HTTP_PROXY` (for HTTP proxy integration). An `HTTP` or `HTTP_PROXY` integration with a `connection_type` of `VPC_LINK` is referred to as a private integration and uses a VpcLink to connect API Gateway to a network load balancer of a VPC.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Input's URI. **Required** if `type` is `AWS`, `AWS_PROXY`, `HTTP` or `HTTP_PROXY`.
        /// For HTTP integrations, the URI must be a fully formed, encoded HTTP(S) URL according to the RFC-3986 specification . For AWS integrations, the URI should be of the form `arn:aws:apigateway:{region}:{subdomain.service|service}:{path|action}/{service_api}`. `region`, `subdomain` and `service` are used to determine the right endpoint.
        /// e.g., `arn:aws:apigateway:eu-west-1:lambda:path/2015-03-31/functions/arn:aws:lambda:eu-west-1:123456789012:function:my-func/invocations`. For private integrations, the URI parameter is not used for routing requests to your endpoint, but is used for setting the Host header and for certificate validation.
        #[builder(into, default)]
        pub uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IntegrationResult {
        /// List of cache key parameters for the integration.
        pub cache_key_parameters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Integration's cache namespace.
        pub cache_namespace: pulumi_gestalt_rust::Output<String>,
        /// ID of the VpcLink used for the integration. **Required** if `connection_type` is `VPC_LINK`
        pub connection_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Integration input's [connectionType](https://docs.aws.amazon.com/apigateway/api-reference/resource/integration/#connectionType). Valid values are `INTERNET` (default for connections through the public routable internet), and `VPC_LINK` (for private connections between API Gateway and a network load balancer in a VPC).
        pub connection_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// How to handle request payload content type conversions. Supported values are `CONVERT_TO_BINARY` and `CONVERT_TO_TEXT`. If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the passthroughBehaviors is configured to support payload pass-through.
        pub content_handling: pulumi_gestalt_rust::Output<Option<String>>,
        /// Credentials required for the integration. For `AWS` integrations, 2 options are available. To specify an IAM Role for Amazon API Gateway to assume, use the role's ARN. To require that the caller's identity be passed through from the request, specify the string `arn:aws:iam::\*:user/\*`.
        pub credentials: pulumi_gestalt_rust::Output<Option<String>>,
        /// HTTP method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTION`, `ANY`)
        /// when calling the associated resource.
        pub http_method: pulumi_gestalt_rust::Output<String>,
        /// Integration HTTP method
        /// (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONs`, `ANY`, `PATCH`) specifying how API Gateway will interact with the back end.
        /// **Required** if `type` is `AWS`, `AWS_PROXY`, `HTTP` or `HTTP_PROXY`.
        /// Not all methods are compatible with all `AWS` integrations.
        /// e.g., Lambda function [can only be invoked](https://github.com/awslabs/aws-apigateway-importer/issues/9#issuecomment-129651005) via `POST`.
        pub integration_http_method: pulumi_gestalt_rust::Output<Option<String>>,
        /// Integration passthrough behavior (`WHEN_NO_MATCH`, `WHEN_NO_TEMPLATES`, `NEVER`).  **Required** if `request_templates` is used.
        pub passthrough_behavior: pulumi_gestalt_rust::Output<String>,
        /// Map of request query string parameters and headers that should be passed to the backend responder.
        /// For example: `request_parameters = { "integration.request.header.X-Some-Other-Header" = "method.request.header.X-Some-Header" }`
        pub request_parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of the integration's request templates.
        pub request_templates: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// API resource ID.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the associated REST API.
        pub rest_api: pulumi_gestalt_rust::Output<String>,
        /// Custom timeout between 50 and 300,000 milliseconds. The default value is 29,000 milliseconds. You need to raise a [Service Quota Ticket](https://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html) to increase time beyond 29,000 milliseconds.
        pub timeout_milliseconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// TLS configuration. See below.
        pub tls_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigateway::IntegrationTlsConfig>,
        >,
        /// Integration input's [type](https://docs.aws.amazon.com/apigateway/api-reference/resource/integration/). Valid values are `HTTP` (for HTTP backends), `MOCK` (not calling any real backend), `AWS` (for AWS services), `AWS_PROXY` (for Lambda proxy integration) and `HTTP_PROXY` (for HTTP proxy integration). An `HTTP` or `HTTP_PROXY` integration with a `connection_type` of `VPC_LINK` is referred to as a private integration and uses a VpcLink to connect API Gateway to a network load balancer of a VPC.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Input's URI. **Required** if `type` is `AWS`, `AWS_PROXY`, `HTTP` or `HTTP_PROXY`.
        /// For HTTP integrations, the URI must be a fully formed, encoded HTTP(S) URL according to the RFC-3986 specification . For AWS integrations, the URI should be of the form `arn:aws:apigateway:{region}:{subdomain.service|service}:{path|action}/{service_api}`. `region`, `subdomain` and `service` are used to determine the right endpoint.
        /// e.g., `arn:aws:apigateway:eu-west-1:lambda:path/2015-03-31/functions/arn:aws:lambda:eu-west-1:123456789012:function:my-func/invocations`. For private integrations, the URI parameter is not used for routing requests to your endpoint, but is used for setting the Host header and for certificate validation.
        pub uri: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationArgs,
    ) -> IntegrationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cache_key_parameters_binding = args.cache_key_parameters.get_output(context);
        let cache_namespace_binding = args.cache_namespace.get_output(context);
        let connection_id_binding = args.connection_id.get_output(context);
        let connection_type_binding = args.connection_type.get_output(context);
        let content_handling_binding = args.content_handling.get_output(context);
        let credentials_binding = args.credentials.get_output(context);
        let http_method_binding = args.http_method.get_output(context);
        let integration_http_method_binding = args
            .integration_http_method
            .get_output(context);
        let passthrough_behavior_binding = args.passthrough_behavior.get_output(context);
        let request_parameters_binding = args.request_parameters.get_output(context);
        let request_templates_binding = args.request_templates.get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let rest_api_binding = args.rest_api.get_output(context);
        let timeout_milliseconds_binding = args.timeout_milliseconds.get_output(context);
        let tls_config_binding = args.tls_config.get_output(context);
        let type__binding = args.type_.get_output(context);
        let uri_binding = args.uri.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/integration:Integration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheKeyParameters".into(),
                    value: &cache_key_parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheNamespace".into(),
                    value: &cache_namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionType".into(),
                    value: &connection_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentHandling".into(),
                    value: &content_handling_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "credentials".into(),
                    value: &credentials_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpMethod".into(),
                    value: &http_method_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationHttpMethod".into(),
                    value: &integration_http_method_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "passthroughBehavior".into(),
                    value: &passthrough_behavior_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestParameters".into(),
                    value: &request_parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestTemplates".into(),
                    value: &request_templates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeoutMilliseconds".into(),
                    value: &timeout_milliseconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsConfig".into(),
                    value: &tls_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uri".into(),
                    value: &uri_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntegrationResult {
            cache_key_parameters: o.get_field("cacheKeyParameters"),
            cache_namespace: o.get_field("cacheNamespace"),
            connection_id: o.get_field("connectionId"),
            connection_type: o.get_field("connectionType"),
            content_handling: o.get_field("contentHandling"),
            credentials: o.get_field("credentials"),
            http_method: o.get_field("httpMethod"),
            integration_http_method: o.get_field("integrationHttpMethod"),
            passthrough_behavior: o.get_field("passthroughBehavior"),
            request_parameters: o.get_field("requestParameters"),
            request_templates: o.get_field("requestTemplates"),
            resource_id: o.get_field("resourceId"),
            rest_api: o.get_field("restApi"),
            timeout_milliseconds: o.get_field("timeoutMilliseconds"),
            tls_config: o.get_field("tlsConfig"),
            type_: o.get_field("type"),
            uri: o.get_field("uri"),
        }
    }
}
