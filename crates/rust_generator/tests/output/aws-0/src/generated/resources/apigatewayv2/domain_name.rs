/// Manages an Amazon API Gateway Version 2 domain name.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html).
///
/// > **Note:** This resource establishes ownership of and the TLS settings for
/// a particular domain name. An API stage can be associated with the domain name using the `aws.apigatewayv2.ApiMapping` resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain_name::create(
///         "example",
///         DomainNameArgs::builder()
///             .domain_name("ws-api.example.com")
///             .domain_name_configuration(
///                 DomainNameDomainNameConfiguration::builder()
///                     .certificateArn("${exampleAwsAcmCertificate.arn}")
///                     .endpointType("REGIONAL")
///                     .securityPolicy("TLS_1_2")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Associated Route 53 Resource Record
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain_name::create(
///         "example",
///         DomainNameArgs::builder()
///             .domain_name("http-api.example.com")
///             .domain_name_configuration(
///                 DomainNameDomainNameConfiguration::builder()
///                     .certificateArn("${exampleAwsAcmCertificate.arn}")
///                     .endpointType("REGIONAL")
///                     .securityPolicy("TLS_1_2")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleRecord = record::create(
///         "exampleRecord",
///         RecordArgs::builder()
///             .aliases(
///                 vec![
///                     RecordAlias::builder().evaluateTargetHealth(false)
///                     .name("${example.domainNameConfiguration.targetDomainName}")
///                     .zoneId("${example.domainNameConfiguration.hostedZoneId}")
///                     .build_struct(),
///                 ],
///             )
///             .name("${example.domainName}")
///             .type_("A")
///             .zone_id("${exampleAwsRoute53Zone.zoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_domain_name` using the domain name. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/domainName:DomainName example ws-api.example.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_name {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainNameArgs {
        /// Domain name. Must be between 1 and 512 characters in length.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Domain name configuration. See below.
        #[builder(into)]
        pub domain_name_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::apigatewayv2::DomainNameDomainNameConfiguration,
        >,
        /// Mutual TLS authentication configuration for the domain name.
        #[builder(into, default)]
        pub mutual_tls_authentication: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigatewayv2::DomainNameMutualTlsAuthentication>,
        >,
        /// Map of tags to assign to the domain name. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainNameResult {
        /// [API mapping selection expression](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-websocket-api-selection-expressions.html#apigateway-websocket-api-mapping-selection-expressions) for the domain name.
        pub api_mapping_selection_expression: pulumi_gestalt_rust::Output<String>,
        /// ARN of the domain name.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Domain name. Must be between 1 and 512 characters in length.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Domain name configuration. See below.
        pub domain_name_configuration: pulumi_gestalt_rust::Output<
            super::super::types::apigatewayv2::DomainNameDomainNameConfiguration,
        >,
        /// Mutual TLS authentication configuration for the domain name.
        pub mutual_tls_authentication: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigatewayv2::DomainNameMutualTlsAuthentication>,
        >,
        /// Map of tags to assign to the domain name. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainNameArgs,
    ) -> DomainNameResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let domain_name_configuration_binding = args
            .domain_name_configuration
            .get_output(context);
        let mutual_tls_authentication_binding = args
            .mutual_tls_authentication
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigatewayv2/domainName:DomainName".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainNameConfiguration".into(),
                    value: &domain_name_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mutualTlsAuthentication".into(),
                    value: &mutual_tls_authentication_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainNameResult {
            api_mapping_selection_expression: o
                .get_field("apiMappingSelectionExpression"),
            arn: o.get_field("arn"),
            domain_name: o.get_field("domainName"),
            domain_name_configuration: o.get_field("domainNameConfiguration"),
            mutual_tls_authentication: o.get_field("mutualTlsAuthentication"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
