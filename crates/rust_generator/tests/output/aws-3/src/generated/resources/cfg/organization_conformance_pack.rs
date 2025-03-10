/// Manages a Config Organization Conformance Pack. More information can be found in the [Managing Conformance Packs Across all Accounts in Your Organization](https://docs.aws.amazon.com/config/latest/developerguide/conformance-pack-organization-apis.html) and [AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html) documentation. Example conformance pack templates may be found in the [AWS Config Rules Repository](https://github.com/awslabs/aws-config-rules/tree/master/aws-config-conformance-packs).
///
/// > **NOTE:** This resource must be created in the Organization master account or a delegated administrator account, and the Organization must have all features enabled. Every Organization account except those configured in the `excluded_accounts` argument must have a Configuration Recorder with proper IAM permissions before the Organization Conformance Pack will successfully create or update. See also the `aws.cfg.Recorder` resource.
///
/// ## Example Usage
///
/// ### Using Template Body
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization_conformance_pack::create(
///         "example",
///         OrganizationConformancePackArgs::builder()
///             .input_parameters(
///                 vec![
///                     OrganizationConformancePackInputParameter::builder()
///                     .parameterName("AccessKeysRotatedParameterMaxAccessKeyAge")
///                     .parameterValue("90").build_struct(),
///                 ],
///             )
///             .name("example")
///             .template_body(
///                 "Parameters:\n  AccessKeysRotatedParameterMaxAccessKeyAge:\n    Type: String\nResources:\n  IAMPasswordPolicy:\n    Properties:\n      ConfigRuleName: IAMPasswordPolicy\n      Source:\n        Owner: AWS\n        SourceIdentifier: IAM_PASSWORD_POLICY\n    Type: AWS::Config::ConfigRule\n",
///             )
///             .build_struct(),
///     );
///     let exampleOrganization = organization::create(
///         "exampleOrganization",
///         OrganizationArgs::builder()
///             .aws_service_access_principals(
///                 vec!["config-multiaccountsetup.amazonaws.com",],
///             )
///             .feature_set("ALL")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using Template S3 URI
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization_conformance_pack::create(
///         "example",
///         OrganizationConformancePackArgs::builder()
///             .name("example")
///             .template_s_3_uri(
///                 "s3://${exampleBucketV2.bucket}/${exampleBucketObjectv2.key}",
///             )
///             .build_struct(),
///     );
///     let exampleBucketObjectv2 = bucket_objectv_2::create(
///         "exampleBucketObjectv2",
///         BucketObjectv2Args::builder()
///             .bucket("${exampleBucketV2.id}")
///             .content(
///                 "Resources:\n  IAMPasswordPolicy:\n    Properties:\n      ConfigRuleName: IAMPasswordPolicy\n      Source:\n        Owner: AWS\n        SourceIdentifier: IAM_PASSWORD_POLICY\n    Type: AWS::Config::ConfigRule",
///             )
///             .key("example-key")
///             .build_struct(),
///     );
///     let exampleBucketV2 = bucket_v_2::create(
///         "exampleBucketV2",
///         BucketV2Args::builder().bucket("example").build_struct(),
///     );
///     let exampleOrganization = organization::create(
///         "exampleOrganization",
///         OrganizationArgs::builder()
///             .aws_service_access_principals(
///                 vec!["config-multiaccountsetup.amazonaws.com",],
///             )
///             .feature_set("ALL")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Config Organization Conformance Packs using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/organizationConformancePack:OrganizationConformancePack example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_conformance_pack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationConformancePackArgs {
        /// Amazon S3 bucket where AWS Config stores conformance pack templates. Delivery bucket must begin with `awsconfigconforms` prefix. Maximum length of 63.
        #[builder(into, default)]
        pub delivery_s3_bucket: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The prefix for the Amazon S3 bucket. Maximum length of 1024.
        #[builder(into, default)]
        pub delivery_s3_key_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of AWS accounts to be excluded from an organization conformance pack while deploying a conformance pack. Maximum of 1000 accounts.
        #[builder(into, default)]
        pub excluded_accounts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Set of configuration blocks describing input parameters passed to the conformance pack template. Documented below. When configured, the parameters must also be included in the `template_body` or in the template stored in Amazon S3 if using `template_s3_uri`.
        #[builder(into, default)]
        pub input_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::cfg::OrganizationConformancePackInputParameter>,
            >,
        >,
        /// The name of the organization conformance pack. Must begin with a letter and contain from 1 to 128 alphanumeric characters and hyphens.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A string containing full conformance pack template body. Maximum length of 51200. Drift detection is not possible with this argument.
        #[builder(into, default)]
        pub template_body: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Location of file, e.g., `s3://bucketname/prefix`, containing the template body. The uri must point to the conformance pack template that is located in an Amazon S3 bucket in the same region as the conformance pack. Maximum length of 1024. Drift detection is not possible with this argument.
        #[builder(into, default)]
        pub template_s3_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationConformancePackResult {
        /// Amazon Resource Name (ARN) of the organization conformance pack.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon S3 bucket where AWS Config stores conformance pack templates. Delivery bucket must begin with `awsconfigconforms` prefix. Maximum length of 63.
        pub delivery_s3_bucket: pulumi_gestalt_rust::Output<Option<String>>,
        /// The prefix for the Amazon S3 bucket. Maximum length of 1024.
        pub delivery_s3_key_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of AWS accounts to be excluded from an organization conformance pack while deploying a conformance pack. Maximum of 1000 accounts.
        pub excluded_accounts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Set of configuration blocks describing input parameters passed to the conformance pack template. Documented below. When configured, the parameters must also be included in the `template_body` or in the template stored in Amazon S3 if using `template_s3_uri`.
        pub input_parameters: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::cfg::OrganizationConformancePackInputParameter>,
            >,
        >,
        /// The name of the organization conformance pack. Must begin with a letter and contain from 1 to 128 alphanumeric characters and hyphens.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A string containing full conformance pack template body. Maximum length of 51200. Drift detection is not possible with this argument.
        pub template_body: pulumi_gestalt_rust::Output<Option<String>>,
        /// Location of file, e.g., `s3://bucketname/prefix`, containing the template body. The uri must point to the conformance pack template that is located in an Amazon S3 bucket in the same region as the conformance pack. Maximum length of 1024. Drift detection is not possible with this argument.
        pub template_s3_uri: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationConformancePackArgs,
    ) -> OrganizationConformancePackResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let delivery_s3_bucket_binding = args.delivery_s3_bucket.get_output(context);
        let delivery_s3_key_prefix_binding = args
            .delivery_s3_key_prefix
            .get_output(context);
        let excluded_accounts_binding = args.excluded_accounts.get_output(context);
        let input_parameters_binding = args.input_parameters.get_output(context);
        let name_binding = args.name.get_output(context);
        let template_body_binding = args.template_body.get_output(context);
        let template_s3_uri_binding = args.template_s3_uri.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cfg/organizationConformancePack:OrganizationConformancePack"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryS3Bucket".into(),
                    value: &delivery_s3_bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryS3KeyPrefix".into(),
                    value: &delivery_s3_key_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludedAccounts".into(),
                    value: &excluded_accounts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputParameters".into(),
                    value: &input_parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateBody".into(),
                    value: &template_body_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateS3Uri".into(),
                    value: &template_s3_uri_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationConformancePackResult {
            arn: o.get_field("arn"),
            delivery_s3_bucket: o.get_field("deliveryS3Bucket"),
            delivery_s3_key_prefix: o.get_field("deliveryS3KeyPrefix"),
            excluded_accounts: o.get_field("excludedAccounts"),
            input_parameters: o.get_field("inputParameters"),
            name: o.get_field("name"),
            template_body: o.get_field("templateBody"),
            template_s3_uri: o.get_field("templateS3Uri"),
        }
    }
}
