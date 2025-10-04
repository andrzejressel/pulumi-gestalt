/// Provides a AWS Transfer AS2 Connector resource.
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
///     let example = connector::create(
///         "example",
///         ConnectorArgs::builder()
///             .access_role("${test.arn}")
///             .as_2_config(
///                 ConnectorAs2Config::builder()
///                     .compression("DISABLED")
///                     .encryptionAlgorithm("AWS128_CBC")
///                     .localProfileId("${local.profileId}")
///                     .mdnResponse("NONE")
///                     .mdnSigningAlgorithm("NONE")
///                     .messageSubject("For Connector")
///                     .partnerProfileId("${partner.profileId}")
///                     .signingAlgorithm("NONE")
///                     .build_struct(),
///             )
///             .url("http://www.test.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### SFTP Connector
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = connector::create(
///         "example",
///         ConnectorArgs::builder()
///             .access_role("${test.arn}")
///             .sftp_config(
///                 ConnectorSftpConfig::builder()
///                     .trustedHostKeys(vec!["ssh-rsa AAAAB3NYourKeysHere",])
///                     .userSecretId("${exampleAwsSecretsmanagerSecret.id}")
///                     .build_struct(),
///             )
///             .url("sftp://test.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transfer AS2 Connector using the `connector_id`. For example:
///
/// ```sh
/// $ pulumi import aws:transfer/connector:Connector example c-4221a88afd5f4362a
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connector {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectorArgs {
        /// The IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
        #[builder(into)]
        pub access_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Either SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
        #[builder(into, default)]
        pub as2_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::transfer::ConnectorAs2Config>,
        >,
        /// The IAM Role which is required for allowing the connector to turn on CloudWatch logging for Amazon S3 events.
        #[builder(into, default)]
        pub logging_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the security policy for the connector.
        #[builder(into, default)]
        pub security_policy_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Either SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
        #[builder(into, default)]
        pub sftp_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::transfer::ConnectorSftpConfig>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The URL of the partners AS2 endpoint or SFTP endpoint.
        #[builder(into)]
        pub url: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectorResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
        pub access_role: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the connector.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Either SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
        pub as2_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::transfer::ConnectorAs2Config>,
        >,
        /// The unique identifier for the AS2 profile or SFTP Profile.
        pub connector_id: pulumi_gestalt_rust::Output<String>,
        /// The IAM Role which is required for allowing the connector to turn on CloudWatch logging for Amazon S3 events.
        pub logging_role: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the security policy for the connector.
        pub security_policy_name: pulumi_gestalt_rust::Output<String>,
        /// Either SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
        pub sftp_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::transfer::ConnectorSftpConfig>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URL of the partners AS2 endpoint or SFTP endpoint.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectorArgs,
    ) -> ConnectorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_role_binding = args.access_role.get_output(context);
        let as2_config_binding = args.as2_config.get_output(context);
        let logging_role_binding = args.logging_role.get_output(context);
        let security_policy_name_binding = args.security_policy_name.get_output(context);
        let sftp_config_binding = args.sftp_config.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let url_binding = args.url.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:transfer/connector:Connector".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessRole".into(),
                    value: &access_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "as2Config".into(),
                    value: &as2_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingRole".into(),
                    value: &logging_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityPolicyName".into(),
                    value: &security_policy_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sftpConfig".into(),
                    value: &sftp_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "url".into(),
                    value: &url_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectorResult {
            id: o.get_field("id"),
            access_role: o.get_field("accessRole"),
            arn: o.get_field("arn"),
            as2_config: o.get_field("as2Config"),
            connector_id: o.get_field("connectorId"),
            logging_role: o.get_field("loggingRole"),
            security_policy_name: o.get_field("securityPolicyName"),
            sftp_config: o.get_field("sftpConfig"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            url: o.get_field("url"),
        }
    }
}
