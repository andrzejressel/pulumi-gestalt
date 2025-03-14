/// Provides a resource to manage the default customer master key (CMK) that your AWS account uses to encrypt EBS volumes.
///
/// Your AWS account has an AWS-managed default CMK that is used for encrypting an EBS volume when no CMK is specified in the API call that creates the volume.
/// By using the `aws.ebs.DefaultKmsKey` resource, you can specify a customer-managed CMK to use in place of the AWS-managed default CMK.
///
/// > **NOTE:** Creating an `aws.ebs.DefaultKmsKey` resource does not enable default EBS encryption. Use the `aws.ebs.EncryptionByDefault` to enable default EBS encryption.
///
/// > **NOTE:** Destroying this resource will reset the default CMK to the account's AWS-managed default CMK for EBS.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = default_kms_key::create(
///         "example",
///         DefaultKmsKeyArgs::builder().key_arn("${exampleAwsKmsKey.arn}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the EBS default KMS CMK using the KMS key ARN. For example:
///
/// ```sh
/// $ pulumi import aws:ebs/defaultKmsKey:DefaultKmsKey example arn:aws:kms:us-east-1:123456789012:key/abcd-1234
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_kms_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultKmsKeyArgs {
        /// The ARN of the AWS Key Management Service (AWS KMS) customer master key (CMK) to use to encrypt the EBS volume.
        #[builder(into)]
        pub key_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultKmsKeyResult {
        /// The ARN of the AWS Key Management Service (AWS KMS) customer master key (CMK) to use to encrypt the EBS volume.
        pub key_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultKmsKeyArgs,
    ) -> DefaultKmsKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let key_arn_binding = args.key_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ebs/defaultKmsKey:DefaultKmsKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyArn".into(),
                    value: &key_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefaultKmsKeyResult {
            key_arn: o.get_field("keyArn"),
        }
    }
}
