/// Provides a AWS Transfer Server resource.
///
/// > **NOTE on AWS IAM permissions:** If the `endpoint_type` is set to `VPC`, the `ec2:DescribeVpcEndpoints` and `ec2:ModifyVpcEndpoint` [actions](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonec2.html#amazonec2-actions-as-permissions) are used.
///
/// > **NOTE:** Use the `aws.transfer.Tag` resource to manage the system tags used for [custom hostnames](https://docs.aws.amazon.com/transfer/latest/userguide/requirements-dns.html#tag-custom-hostname-cdk).
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:transfer:Server
///     properties:
///       tags:
///         Name: Example
/// ```
///
/// ### Security Policy Name
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = server::create(
///         "example",
///         ServerArgs::builder()
///             .security_policy_name("TransferSecurityPolicy-2020-06")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### VPC Endpoint
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = server::create(
///         "example",
///         ServerArgs::builder()
///             .endpoint_details(
///                 ServerEndpointDetails::builder()
///                     .addressAllocationIds(vec!["${exampleAwsEip.id}",])
///                     .subnetIds(vec!["${exampleAwsSubnet.id}",])
///                     .vpcId("${exampleAwsVpc.id}")
///                     .build_struct(),
///             )
///             .endpoint_type("VPC")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### AWS Directory authentication
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = server::create(
///         "example",
///         ServerArgs::builder()
///             .directory_id("${exampleAwsDirectoryServiceDirectory.id}")
///             .identity_provider_type("AWS_DIRECTORY_SERVICE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### AWS Lambda authentication
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = server::create(
///         "example",
///         ServerArgs::builder()
///             .function("${exampleAwsLambdaIdentityProvider.arn}")
///             .identity_provider_type("AWS_LAMBDA")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Protocols
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = server::create(
///         "example",
///         ServerArgs::builder()
///             .certificate("${exampleAwsAcmCertificate.arn}")
///             .endpoint_details(
///                 ServerEndpointDetails::builder()
///                     .subnetIds(vec!["${exampleAwsSubnet.id}",])
///                     .vpcId("${exampleAwsVpc.id}")
///                     .build_struct(),
///             )
///             .endpoint_type("VPC")
///             .identity_provider_type("API_GATEWAY")
///             .protocols(vec!["FTP", "FTPS",])
///             .url(
///                 "${exampleAwsApiGatewayDeployment.invokeUrl}${exampleAwsApiGatewayResource.path}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using Structured Logging Destinations
///
/// ```yaml
/// resources:
///   transfer:
///     type: aws:cloudwatch:LogGroup
///     properties:
///       namePrefix: transfer_test_
///   iamForTransfer:
///     type: aws:iam:Role
///     name: iam_for_transfer
///     properties:
///       namePrefix: iam_for_transfer_
///       assumeRolePolicy: ${transferAssumeRole.json}
///       managedPolicyArns:
///         - arn:aws:iam::aws:policy/service-role/AWSTransferLoggingAccess
///   transferServer:
///     type: aws:transfer:Server
///     name: transfer
///     properties:
///       endpointType: PUBLIC
///       loggingRole: ${iamForTransfer.arn}
///       protocols:
///         - SFTP
///       structuredLogDestinations:
///         - ${transfer.arn}:*
/// variables:
///   transferAssumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - transfer.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ## Import
///
/// In Terraform v1.5.0 and later, use an `import` Block to import Transfer Servers using the server `id`. For example:
///
/// Using `pulumi import`, import Transfer Servers using the server `id`. For example:
///
/// ```sh
/// $ pulumi import aws:transfer/server:Server example s-12345678
/// ```
/// Certain resource arguments, such as `host_key`, cannot be read via the API and imported into the provider. This provider will display a difference for these arguments the first run after import if declared in the provider configuration for an imported resource.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerArgs {
        /// The Amazon Resource Name (ARN) of the AWS Certificate Manager (ACM) certificate. This is required when `protocols` is set to `FTPS`
        #[builder(into, default)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The directory service ID of the directory service you want to connect to with an `identity_provider_type` of `AWS_DIRECTORY_SERVICE`.
        #[builder(into, default)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The domain of the storage system that is used for file transfers. Valid values are: `S3` and `EFS`. The default value is `S3`.
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The virtual private cloud (VPC) endpoint settings that you want to configure for your SFTP server. See `endpoint_details` Block below for details.
        #[builder(into, default)]
        pub endpoint_details: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::transfer::ServerEndpointDetails>,
        >,
        /// The type of endpoint that you want your SFTP server connect to. If you connect to a `VPC` (or `VPC_ENDPOINT`), your SFTP server isn't accessible over the public internet. If you want to connect your SFTP server via public internet, set `PUBLIC`.  Defaults to `PUBLIC`.
        #[builder(into, default)]
        pub endpoint_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A boolean that indicates all users associated with the server should be deleted so that the Server can be destroyed without error. The default value is `false`. This option only applies to servers configured with a `SERVICE_MANAGED` `identity_provider_type`.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ARN for a lambda function to use for the Identity provider.
        #[builder(into, default)]
        pub function: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// RSA, ECDSA, or ED25519 private key (e.g., as generated by the `ssh-keygen -t rsa -b 2048 -N "" -m PEM -f my-new-server-key`, `ssh-keygen -t ecdsa -b 256 -N "" -m PEM -f my-new-server-key` or `ssh-keygen -t ed25519 -N "" -f my-new-server-key` commands).
        #[builder(into, default)]
        pub host_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The mode of authentication enabled for this service. The default value is `SERVICE_MANAGED`, which allows you to store and access SFTP user credentials within the service. `API_GATEWAY` indicates that user authentication requires a call to an API Gateway endpoint URL provided by you to integrate an identity provider of your choice. Using `AWS_DIRECTORY_SERVICE` will allow for authentication against AWS Managed Active Directory or Microsoft Active Directory in your on-premises environment, or in AWS using AD Connectors. Use the `AWS_LAMBDA` value to directly use a Lambda function as your identity provider. If you choose this value, you must specify the ARN for the lambda function in the `function` argument.
        #[builder(into, default)]
        pub identity_provider_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the IAM role used to authenticate the user account with an `identity_provider_type` of `API_GATEWAY`.
        #[builder(into, default)]
        pub invocation_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of an IAM role that allows the service to write your SFTP users’ activity to your Amazon CloudWatch logs for monitoring and auditing purposes.
        #[builder(into, default)]
        pub logging_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify a string to display when users connect to a server. This string is displayed after the user authenticates. The SFTP protocol does not support post-authentication display banners.
        #[builder(into, default)]
        pub post_authentication_login_banner: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specify a string to display when users connect to a server. This string is displayed before the user authenticates.
        #[builder(into, default)]
        pub pre_authentication_login_banner: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The protocol settings that are configured for your server. See `protocol_details` Block below for details.
        #[builder(into, default)]
        pub protocol_details: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::transfer::ServerProtocolDetails>,
        >,
        /// Specifies the file transfer protocol or protocols over which your file transfer protocol client can connect to your server's endpoint. This defaults to `SFTP` . The available protocols are:
        /// * `AS2`: File transfer over Applicability Statement 2
        /// * `SFTP`: File transfer over SSH
        /// * `FTPS`: File transfer with TLS encryption
        /// * `FTP`: Unencrypted file transfer
        #[builder(into, default)]
        pub protocols: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies whether or not performance for your Amazon S3 directories is optimized. This is disabled by default. See `s3_storage_options` Block below for details.
        #[builder(into, default)]
        pub s3_storage_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::transfer::ServerS3StorageOptions>,
        >,
        /// Specifies the name of the security policy that is attached to the server. Default value is: `TransferSecurityPolicy-2018-11`. The available values are:
        /// * `TransferSecurityPolicy-2018-11`
        /// * `TransferSecurityPolicy-2020-06`
        /// * `TransferSecurityPolicy-2022-03`
        /// * `TransferSecurityPolicy-2023-05`
        /// * `TransferSecurityPolicy-2024-01`
        /// * `TransferSecurityPolicy-FIPS-2020-06`
        /// * `TransferSecurityPolicy-FIPS-2023-05`
        /// * `TransferSecurityPolicy-FIPS-2024-01`
        /// * `TransferSecurityPolicy-FIPS-2024-05`
        /// * `TransferSecurityPolicy-PQ-SSH-Experimental-2023-04`
        /// * `TransferSecurityPolicy-PQ-SSH-FIPS-Experimental-2023-04`
        /// * `TransferSecurityPolicy-Restricted-2018-11`
        /// * `TransferSecurityPolicy-Restricted-2020-06`
        /// * `TransferSecurityPolicy-Restricted-2024-06`
        ///
        /// See [Security policies for AWS Transfer Family servers](https://docs.aws.amazon.com/transfer/latest/userguide/security-policies.html) for details.
        #[builder(into, default)]
        pub security_policy_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// For SFTP-enabled servers, and for custom identity providers only. Valid values are `PASSWORD`, `PUBLIC_KEY`, `PUBLIC_KEY_OR_PASSWORD` and `PUBLIC_KEY_AND_PASSWORD`. Default value is: `PUBLIC_KEY_OR_PASSWORD`.
        #[builder(into, default)]
        pub sftp_authentication_methods: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A set of ARNs of destinations that will receive structured logs from the transfer server such as CloudWatch Log Group ARNs. If provided this enables the transfer server to emit structured logs to the specified locations.
        #[builder(into, default)]
        pub structured_log_destinations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// URL of the service endpoint used to authenticate users with an `identity_provider_type` of `API_GATEWAY`.
        #[builder(into, default)]
        pub url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the workflow details. See `workflow_details` Block below for details.
        #[builder(into, default)]
        pub workflow_details: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::transfer::ServerWorkflowDetails>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of Transfer Server
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the AWS Certificate Manager (ACM) certificate. This is required when `protocols` is set to `FTPS`
        pub certificate: pulumi_gestalt_rust::Output<Option<String>>,
        /// The directory service ID of the directory service you want to connect to with an `identity_provider_type` of `AWS_DIRECTORY_SERVICE`.
        pub directory_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The domain of the storage system that is used for file transfers. Valid values are: `S3` and `EFS`. The default value is `S3`.
        pub domain: pulumi_gestalt_rust::Output<Option<String>>,
        /// The endpoint of the Transfer Server (e.g., `s-12345678.server.transfer.REGION.amazonaws.com`)
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The virtual private cloud (VPC) endpoint settings that you want to configure for your SFTP server. See `endpoint_details` Block below for details.
        pub endpoint_details: pulumi_gestalt_rust::Output<
            Option<super::super::types::transfer::ServerEndpointDetails>,
        >,
        /// The type of endpoint that you want your SFTP server connect to. If you connect to a `VPC` (or `VPC_ENDPOINT`), your SFTP server isn't accessible over the public internet. If you want to connect your SFTP server via public internet, set `PUBLIC`.  Defaults to `PUBLIC`.
        pub endpoint_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A boolean that indicates all users associated with the server should be deleted so that the Server can be destroyed without error. The default value is `false`. This option only applies to servers configured with a `SERVICE_MANAGED` `identity_provider_type`.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ARN for a lambda function to use for the Identity provider.
        pub function: pulumi_gestalt_rust::Output<Option<String>>,
        /// RSA, ECDSA, or ED25519 private key (e.g., as generated by the `ssh-keygen -t rsa -b 2048 -N "" -m PEM -f my-new-server-key`, `ssh-keygen -t ecdsa -b 256 -N "" -m PEM -f my-new-server-key` or `ssh-keygen -t ed25519 -N "" -f my-new-server-key` commands).
        pub host_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// This value contains the message-digest algorithm (MD5) hash of the server's host key. This value is equivalent to the output of the `ssh-keygen -l -E md5 -f my-new-server-key` command.
        pub host_key_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The mode of authentication enabled for this service. The default value is `SERVICE_MANAGED`, which allows you to store and access SFTP user credentials within the service. `API_GATEWAY` indicates that user authentication requires a call to an API Gateway endpoint URL provided by you to integrate an identity provider of your choice. Using `AWS_DIRECTORY_SERVICE` will allow for authentication against AWS Managed Active Directory or Microsoft Active Directory in your on-premises environment, or in AWS using AD Connectors. Use the `AWS_LAMBDA` value to directly use a Lambda function as your identity provider. If you choose this value, you must specify the ARN for the lambda function in the `function` argument.
        pub identity_provider_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the IAM role used to authenticate the user account with an `identity_provider_type` of `API_GATEWAY`.
        pub invocation_role: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of an IAM role that allows the service to write your SFTP users’ activity to your Amazon CloudWatch logs for monitoring and auditing purposes.
        pub logging_role: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specify a string to display when users connect to a server. This string is displayed after the user authenticates. The SFTP protocol does not support post-authentication display banners.
        pub post_authentication_login_banner: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Specify a string to display when users connect to a server. This string is displayed before the user authenticates.
        pub pre_authentication_login_banner: pulumi_gestalt_rust::Output<Option<String>>,
        /// The protocol settings that are configured for your server. See `protocol_details` Block below for details.
        pub protocol_details: pulumi_gestalt_rust::Output<
            super::super::types::transfer::ServerProtocolDetails,
        >,
        /// Specifies the file transfer protocol or protocols over which your file transfer protocol client can connect to your server's endpoint. This defaults to `SFTP` . The available protocols are:
        /// * `AS2`: File transfer over Applicability Statement 2
        /// * `SFTP`: File transfer over SSH
        /// * `FTPS`: File transfer with TLS encryption
        /// * `FTP`: Unencrypted file transfer
        pub protocols: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies whether or not performance for your Amazon S3 directories is optimized. This is disabled by default. See `s3_storage_options` Block below for details.
        pub s3_storage_options: pulumi_gestalt_rust::Output<
            super::super::types::transfer::ServerS3StorageOptions,
        >,
        /// Specifies the name of the security policy that is attached to the server. Default value is: `TransferSecurityPolicy-2018-11`. The available values are:
        /// * `TransferSecurityPolicy-2018-11`
        /// * `TransferSecurityPolicy-2020-06`
        /// * `TransferSecurityPolicy-2022-03`
        /// * `TransferSecurityPolicy-2023-05`
        /// * `TransferSecurityPolicy-2024-01`
        /// * `TransferSecurityPolicy-FIPS-2020-06`
        /// * `TransferSecurityPolicy-FIPS-2023-05`
        /// * `TransferSecurityPolicy-FIPS-2024-01`
        /// * `TransferSecurityPolicy-FIPS-2024-05`
        /// * `TransferSecurityPolicy-PQ-SSH-Experimental-2023-04`
        /// * `TransferSecurityPolicy-PQ-SSH-FIPS-Experimental-2023-04`
        /// * `TransferSecurityPolicy-Restricted-2018-11`
        /// * `TransferSecurityPolicy-Restricted-2020-06`
        /// * `TransferSecurityPolicy-Restricted-2024-06`
        ///
        /// See [Security policies for AWS Transfer Family servers](https://docs.aws.amazon.com/transfer/latest/userguide/security-policies.html) for details.
        pub security_policy_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// For SFTP-enabled servers, and for custom identity providers only. Valid values are `PASSWORD`, `PUBLIC_KEY`, `PUBLIC_KEY_OR_PASSWORD` and `PUBLIC_KEY_AND_PASSWORD`. Default value is: `PUBLIC_KEY_OR_PASSWORD`.
        pub sftp_authentication_methods: pulumi_gestalt_rust::Output<String>,
        /// A set of ARNs of destinations that will receive structured logs from the transfer server such as CloudWatch Log Group ARNs. If provided this enables the transfer server to emit structured logs to the specified locations.
        pub structured_log_destinations: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// URL of the service endpoint used to authenticate users with an `identity_provider_type` of `API_GATEWAY`.
        pub url: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the workflow details. See `workflow_details` Block below for details.
        pub workflow_details: pulumi_gestalt_rust::Output<
            Option<super::super::types::transfer::ServerWorkflowDetails>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerArgs,
    ) -> ServerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_binding = args.certificate.get_output(context);
        let directory_id_binding = args.directory_id.get_output(context);
        let domain_binding = args.domain.get_output(context);
        let endpoint_details_binding = args.endpoint_details.get_output(context);
        let endpoint_type_binding = args.endpoint_type.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let function_binding = args.function.get_output(context);
        let host_key_binding = args.host_key.get_output(context);
        let identity_provider_type_binding = args
            .identity_provider_type
            .get_output(context);
        let invocation_role_binding = args.invocation_role.get_output(context);
        let logging_role_binding = args.logging_role.get_output(context);
        let post_authentication_login_banner_binding = args
            .post_authentication_login_banner
            .get_output(context);
        let pre_authentication_login_banner_binding = args
            .pre_authentication_login_banner
            .get_output(context);
        let protocol_details_binding = args.protocol_details.get_output(context);
        let protocols_binding = args.protocols.get_output(context);
        let s3_storage_options_binding = args.s3_storage_options.get_output(context);
        let security_policy_name_binding = args.security_policy_name.get_output(context);
        let sftp_authentication_methods_binding = args
            .sftp_authentication_methods
            .get_output(context);
        let structured_log_destinations_binding = args
            .structured_log_destinations
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let url_binding = args.url.get_output(context);
        let workflow_details_binding = args.workflow_details.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:transfer/server:Server".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: &domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointDetails".into(),
                    value: &endpoint_details_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointType".into(),
                    value: &endpoint_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "function".into(),
                    value: &function_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostKey".into(),
                    value: &host_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityProviderType".into(),
                    value: &identity_provider_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "invocationRole".into(),
                    value: &invocation_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingRole".into(),
                    value: &logging_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "postAuthenticationLoginBanner".into(),
                    value: &post_authentication_login_banner_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preAuthenticationLoginBanner".into(),
                    value: &pre_authentication_login_banner_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocolDetails".into(),
                    value: &protocol_details_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocols".into(),
                    value: &protocols_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "s3StorageOptions".into(),
                    value: &s3_storage_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityPolicyName".into(),
                    value: &security_policy_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sftpAuthenticationMethods".into(),
                    value: &sftp_authentication_methods_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "structuredLogDestinations".into(),
                    value: &structured_log_destinations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "url".into(),
                    value: &url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workflowDetails".into(),
                    value: &workflow_details_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServerResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            certificate: o.get_field("certificate"),
            directory_id: o.get_field("directoryId"),
            domain: o.get_field("domain"),
            endpoint: o.get_field("endpoint"),
            endpoint_details: o.get_field("endpointDetails"),
            endpoint_type: o.get_field("endpointType"),
            force_destroy: o.get_field("forceDestroy"),
            function: o.get_field("function"),
            host_key: o.get_field("hostKey"),
            host_key_fingerprint: o.get_field("hostKeyFingerprint"),
            identity_provider_type: o.get_field("identityProviderType"),
            invocation_role: o.get_field("invocationRole"),
            logging_role: o.get_field("loggingRole"),
            post_authentication_login_banner: o
                .get_field("postAuthenticationLoginBanner"),
            pre_authentication_login_banner: o.get_field("preAuthenticationLoginBanner"),
            protocol_details: o.get_field("protocolDetails"),
            protocols: o.get_field("protocols"),
            s3_storage_options: o.get_field("s3StorageOptions"),
            security_policy_name: o.get_field("securityPolicyName"),
            sftp_authentication_methods: o.get_field("sftpAuthenticationMethods"),
            structured_log_destinations: o.get_field("structuredLogDestinations"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            url: o.get_field("url"),
            workflow_details: o.get_field("workflowDetails"),
        }
    }
}
