/// Provides an AppConfig Configuration Profile resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appconfig:ConfigurationProfile
///     properties:
///       applicationId: ${exampleAwsAppconfigApplication.id}
///       description: Example Configuration Profile
///       name: example-configuration-profile-tf
///       locationUri: hosted
///       validators:
///         - content: ${exampleAwsLambdaFunction.arn}
///           type: LAMBDA
///       tags:
///         Type: AppConfig Configuration Profile
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppConfig Configuration Profiles using the configuration profile ID and application ID separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:appconfig/configurationProfile:ConfigurationProfile example 71abcde:11xxxxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configuration_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationProfileArgs {
        /// Application ID. Must be between 4 and 7 characters in length.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the configuration profile. Can be at most 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier for an Key Management Service key to encrypt new configuration data versions in the AppConfig hosted configuration store. This attribute is only used for hosted configuration types. The identifier can be an KMS key ID, alias, or the Amazon Resource Name (ARN) of the key ID or alias.
        #[builder(into, default)]
        pub kms_key_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URI to locate the configuration. You can specify the AWS AppConfig hosted configuration store, Systems Manager (SSM) document, an SSM Parameter Store parameter, or an Amazon S3 object. For the hosted configuration store, specify `hosted`. For an SSM document, specify either the document name in the format `ssm-document://<Document_name>` or the ARN. For a parameter, specify either the parameter name in the format `ssm-parameter://<Parameter_name>` or the ARN. For an Amazon S3 object, specify the URI in the following format: `s3://<bucket>/<objectKey>`.
        #[builder(into)]
        pub location_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name for the configuration profile. Must be between 1 and 128 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of an IAM role with permission to access the configuration at the specified `location_uri`. A retrieval role ARN is not required for configurations stored in the AWS AppConfig `hosted` configuration store. It is required for all other sources that store your configuration.
        #[builder(into, default)]
        pub retrieval_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of configurations contained in the profile. Valid values: `AWS.AppConfig.FeatureFlags` and `AWS.Freeform`.  Default: `AWS.Freeform`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of methods for validating the configuration. Maximum of 2. See Validator below for more details.
        #[builder(into, default)]
        pub validators: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appconfig::ConfigurationProfileValidator>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationProfileResult {
        /// Application ID. Must be between 4 and 7 characters in length.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the AppConfig Configuration Profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The configuration profile ID.
        pub configuration_profile_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the configuration profile. Can be at most 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The identifier for an Key Management Service key to encrypt new configuration data versions in the AppConfig hosted configuration store. This attribute is only used for hosted configuration types. The identifier can be an KMS key ID, alias, or the Amazon Resource Name (ARN) of the key ID or alias.
        pub kms_key_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// URI to locate the configuration. You can specify the AWS AppConfig hosted configuration store, Systems Manager (SSM) document, an SSM Parameter Store parameter, or an Amazon S3 object. For the hosted configuration store, specify `hosted`. For an SSM document, specify either the document name in the format `ssm-document://<Document_name>` or the ARN. For a parameter, specify either the parameter name in the format `ssm-parameter://<Parameter_name>` or the ARN. For an Amazon S3 object, specify the URI in the following format: `s3://<bucket>/<objectKey>`.
        pub location_uri: pulumi_gestalt_rust::Output<String>,
        /// Name for the configuration profile. Must be between 1 and 128 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ARN of an IAM role with permission to access the configuration at the specified `location_uri`. A retrieval role ARN is not required for configurations stored in the AWS AppConfig `hosted` configuration store. It is required for all other sources that store your configuration.
        pub retrieval_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of configurations contained in the profile. Valid values: `AWS.AppConfig.FeatureFlags` and `AWS.Freeform`.  Default: `AWS.Freeform`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of methods for validating the configuration. Maximum of 2. See Validator below for more details.
        pub validators: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::appconfig::ConfigurationProfileValidator>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigurationProfileArgs,
    ) -> ConfigurationProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let kms_key_identifier_binding = args.kms_key_identifier.get_output(context);
        let location_uri_binding = args.location_uri.get_output(context);
        let name_binding = args.name.get_output(context);
        let retrieval_role_arn_binding = args.retrieval_role_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let validators_binding = args.validators.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appconfig/configurationProfile:ConfigurationProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyIdentifier".into(),
                    value: &kms_key_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "locationUri".into(),
                    value: &location_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retrievalRoleArn".into(),
                    value: &retrieval_role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validators".into(),
                    value: &validators_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfigurationProfileResult {
            application_id: o.get_field("applicationId"),
            arn: o.get_field("arn"),
            configuration_profile_id: o.get_field("configurationProfileId"),
            description: o.get_field("description"),
            kms_key_identifier: o.get_field("kmsKeyIdentifier"),
            location_uri: o.get_field("locationUri"),
            name: o.get_field("name"),
            retrieval_role_arn: o.get_field("retrievalRoleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
            validators: o.get_field("validators"),
        }
    }
}
