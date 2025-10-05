/// Resource for managing an AWS FinSpace Kx Environment.
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
///     let example = key::create(
///         "example",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Sample KMS Key")
///             .build_struct(),
///     );
///     let exampleKxEnvironment = kx_environment::create(
///         "exampleKxEnvironment",
///         KxEnvironmentArgs::builder()
///             .kms_key_id("${example.arn}")
///             .name("my-tf-kx-environment")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Transit Gateway Configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key::create(
///         "example",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Sample KMS Key")
///             .build_struct(),
///     );
///     let exampleEnv = kx_environment::create(
///         "exampleEnv",
///         KxEnvironmentArgs::builder()
///             .custom_dns_configurations(
///                 vec![
///                     KxEnvironmentCustomDnsConfiguration::builder()
///                     .customDnsServerIp("10.0.0.76")
///                     .customDnsServerName("example.finspace.amazonaws.com")
///                     .build_struct(),
///                 ],
///             )
///             .description("Environment description")
///             .kms_key_id("${example.arn}")
///             .name("my-tf-kx-environment")
///             .transit_gateway_configuration(
///                 KxEnvironmentTransitGatewayConfiguration::builder()
///                     .routableCidrSpace("100.64.0.0/26")
///                     .transitGatewayId("${exampleTransitGateway.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleTransitGateway = transit_gateway::create(
///         "exampleTransitGateway",
///         TransitGatewayArgs::builder().description("example").build_struct(),
///     );
/// }
/// ```
///
/// ### With Transit Gateway Attachment Network ACL Configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key::create(
///         "example",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Sample KMS Key")
///             .build_struct(),
///     );
///     let exampleEnv = kx_environment::create(
///         "exampleEnv",
///         KxEnvironmentArgs::builder()
///             .custom_dns_configurations(
///                 vec![
///                     KxEnvironmentCustomDnsConfiguration::builder()
///                     .customDnsServerIp("10.0.0.76")
///                     .customDnsServerName("example.finspace.amazonaws.com")
///                     .build_struct(),
///                 ],
///             )
///             .description("Environment description")
///             .kms_key_id("${example.arn}")
///             .name("my-tf-kx-environment")
///             .transit_gateway_configuration(
///                 KxEnvironmentTransitGatewayConfiguration::builder()
///                     .attachmentNetworkAclConfigurations(
///                         vec![
///                             KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfiguration::builder()
///                             .cidrBlock("0.0.0.0/0")
///                             .icmpTypeCode(KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfigurationIcmpTypeCode::builder()
///                             .code(- 1). type (- 1).build_struct())
///                             .portRange(KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfigurationPortRange::builder()
///                             .from(53).to(53).build_struct()).protocol("6")
///                             .ruleAction("allow").ruleNumber(1).build_struct(),
///                         ],
///                     )
///                     .routableCidrSpace("100.64.0.0/26")
///                     .transitGatewayId("${exampleTransitGateway.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleTransitGateway = transit_gateway::create(
///         "exampleTransitGateway",
///         TransitGatewayArgs::builder().description("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an AWS FinSpace Kx Environment using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:finspace/kxEnvironment:KxEnvironment example n3ceo7wqxoxcti5tujqwzs
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kx_environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxEnvironmentArgs {
        /// List of DNS server name and server IP. This is used to set up Route-53 outbound resolvers. Defined below.
        #[builder(into, default)]
        pub custom_dns_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::finspace::KxEnvironmentCustomDnsConfiguration>,
            >,
        >,
        /// Description for the KX environment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// KMS key ID to encrypt your data in the FinSpace environment.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the KX environment that you want to create.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Transit gateway and network configuration that is used to connect the KX environment to an internal network. Defined below.
        #[builder(into, default)]
        pub transit_gateway_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::finspace::KxEnvironmentTransitGatewayConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct KxEnvironmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) identifier of the KX environment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS Availability Zone IDs that this environment is available in. Important when selecting VPC subnets to use in cluster creation.
        pub availability_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Timestamp at which the environment is created in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub created_timestamp: pulumi_gestalt_rust::Output<String>,
        /// List of DNS server name and server IP. This is used to set up Route-53 outbound resolvers. Defined below.
        pub custom_dns_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::finspace::KxEnvironmentCustomDnsConfiguration>,
            >,
        >,
        /// Description for the KX environment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Unique identifier for the AWS environment infrastructure account.
        pub infrastructure_account_id: pulumi_gestalt_rust::Output<String>,
        /// KMS key ID to encrypt your data in the FinSpace environment.
        ///
        /// The following arguments are optional:
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Last timestamp at which the environment was updated in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub last_modified_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Name of the KX environment that you want to create.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Status of environment creation
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Transit gateway and network configuration that is used to connect the KX environment to an internal network. Defined below.
        pub transit_gateway_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::finspace::KxEnvironmentTransitGatewayConfiguration,
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
        args: KxEnvironmentArgs,
    ) -> KxEnvironmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_dns_configurations_binding = args
            .custom_dns_configurations
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transit_gateway_configuration_binding = args
            .transit_gateway_configuration
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:finspace/kxEnvironment:KxEnvironment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customDnsConfigurations".into(),
                    value: &custom_dns_configurations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transitGatewayConfiguration".into(),
                    value: &transit_gateway_configuration_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        KxEnvironmentResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            availability_zones: o.get_field("availabilityZones"),
            created_timestamp: o.get_field("createdTimestamp"),
            custom_dns_configurations: o.get_field("customDnsConfigurations"),
            description: o.get_field("description"),
            infrastructure_account_id: o.get_field("infrastructureAccountId"),
            kms_key_id: o.get_field("kmsKeyId"),
            last_modified_timestamp: o.get_field("lastModifiedTimestamp"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            transit_gateway_configuration: o.get_field("transitGatewayConfiguration"),
        }
    }
}
