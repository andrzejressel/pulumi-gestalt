/// Provides an EventBridge event API Destination resource.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = event_api_destination::create(
///         "test",
///         EventApiDestinationArgs::builder()
///             .connection_arn("${testAwsCloudwatchEventConnection.arn}")
///             .description("An API Destination")
///             .http_method("POST")
///             .invocation_endpoint("https://api.destination.com/endpoint")
///             .invocation_rate_limit_per_second(20)
///             .name("api-destination")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EventBridge API Destinations using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/eventApiDestination:EventApiDestination test api-destination
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_api_destination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventApiDestinationArgs {
        /// ARN of the EventBridge Connection to use for the API Destination.
        #[builder(into)]
        pub connection_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the new API Destination. Maximum of 512 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Select the HTTP method used for the invocation endpoint, such as GET, POST, PUT, etc.
        #[builder(into)]
        pub http_method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// URL endpoint to invoke as a target. This could be a valid endpoint generated by a partner service. You can include "*" as path parameters wildcards to be set from the Target HttpParameters.
        #[builder(into)]
        pub invocation_endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Enter the maximum number of invocations per second to allow for this destination. Enter a value greater than 0 (default 300).
        #[builder(into, default)]
        pub invocation_rate_limit_per_second: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The name of the new API Destination. The name must be unique for your account. Maximum of 64 characters consisting of numbers, lower/upper case letters, .,-,_.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EventApiDestinationResult {
        /// The Amazon Resource Name (ARN) of the event API Destination.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the EventBridge Connection to use for the API Destination.
        pub connection_arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the new API Destination. Maximum of 512 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Select the HTTP method used for the invocation endpoint, such as GET, POST, PUT, etc.
        pub http_method: pulumi_gestalt_rust::Output<String>,
        /// URL endpoint to invoke as a target. This could be a valid endpoint generated by a partner service. You can include "*" as path parameters wildcards to be set from the Target HttpParameters.
        pub invocation_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Enter the maximum number of invocations per second to allow for this destination. Enter a value greater than 0 (default 300).
        pub invocation_rate_limit_per_second: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the new API Destination. The name must be unique for your account. Maximum of 64 characters consisting of numbers, lower/upper case letters, .,-,_.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventApiDestinationArgs,
    ) -> EventApiDestinationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_arn_binding = args.connection_arn.get_output(context);
        let description_binding = args.description.get_output(context);
        let http_method_binding = args.http_method.get_output(context);
        let invocation_endpoint_binding = args.invocation_endpoint.get_output(context);
        let invocation_rate_limit_per_second_binding = args
            .invocation_rate_limit_per_second
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventApiDestination:EventApiDestination".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionArn".into(),
                    value: &connection_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpMethod".into(),
                    value: &http_method_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "invocationEndpoint".into(),
                    value: &invocation_endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "invocationRateLimitPerSecond".into(),
                    value: &invocation_rate_limit_per_second_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventApiDestinationResult {
            arn: o.get_field("arn"),
            connection_arn: o.get_field("connectionArn"),
            description: o.get_field("description"),
            http_method: o.get_field("httpMethod"),
            invocation_endpoint: o.get_field("invocationEndpoint"),
            invocation_rate_limit_per_second: o
                .get_field("invocationRateLimitPerSecond"),
            name: o.get_field("name"),
        }
    }
}
