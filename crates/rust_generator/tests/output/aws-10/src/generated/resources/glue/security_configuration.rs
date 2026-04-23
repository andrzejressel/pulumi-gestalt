/// Manages a Glue Security Configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_configuration::create(
///         "example",
///         SecurityConfigurationArgs::builder()
///             .encryption_configuration(
///                 SecurityConfigurationEncryptionConfiguration::builder()
///                     .cloudwatchEncryption(
///                         SecurityConfigurationEncryptionConfigurationCloudwatchEncryption::builder()
///                             .cloudwatchEncryptionMode("DISABLED")
///                             .build_struct(),
///                     )
///                     .jobBookmarksEncryption(
///                         SecurityConfigurationEncryptionConfigurationJobBookmarksEncryption::builder()
///                             .jobBookmarksEncryptionMode("DISABLED")
///                             .build_struct(),
///                     )
///                     .s3Encryption(
///                         SecurityConfigurationEncryptionConfigurationS3Encryption::builder()
///                             .kmsKeyArn("${exampleAwsKmsKey.arn}")
///                             .s3EncryptionMode("SSE-KMS")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Security Configurations using `name`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/securityConfiguration:SecurityConfiguration example example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod security_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityConfigurationArgs {
        /// Configuration block containing encryption configuration. Detailed below.
        #[builder(into)]
        pub encryption_configuration: pulumi_gestalt_rust::Input<
            super::super::types::glue::SecurityConfigurationEncryptionConfiguration,
        >,
        /// Name of the security configuration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecurityConfigurationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block containing encryption configuration. Detailed below.
        pub encryption_configuration: pulumi_gestalt_rust::Output<
            super::super::types::glue::SecurityConfigurationEncryptionConfiguration,
        >,
        /// Name of the security configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityConfigurationArgs,
    ) -> SecurityConfigurationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityConfigurationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SecurityConfigurationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityConfigurationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SecurityConfigurationResult {
        let encryption_configuration_binding = args
            .encryption_configuration
            .get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/securityConfiguration:SecurityConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SecurityConfigurationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            encryption_configuration: o.get_field("encryptionConfiguration"),
            name: o.get_field("name"),
        }
    }
}
