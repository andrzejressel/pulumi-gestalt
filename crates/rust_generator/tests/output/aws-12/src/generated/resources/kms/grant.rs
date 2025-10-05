/// Provides a resource-based access control mechanism for a KMS customer master key.
///
/// > **Note:** All arguments including the grant token will be stored in the raw state as plain-text.
/// ## Import
///
/// Using `pulumi import`, import KMS Grants using the Key ID and Grant ID separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:kms/grant:Grant test 1234abcd-12ab-34cd-56ef-1234567890ab:abcde1237f76e4ba7987489ac329fbfba6ad343d6f7075dbd1ef191f0120514
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod grant {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GrantArgs {
        /// A structure that you can use to allow certain operations in the grant only when the desired encryption context is present. For more information about encryption context, see [Encryption Context](http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html).
        #[builder(into, default)]
        pub constraints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::kms::GrantConstraint>>,
        >,
        /// A list of grant tokens to be used when creating the grant. See [Grant Tokens](http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token) for more information about grant tokens.
        #[builder(into, default)]
        pub grant_creation_tokens: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The principal that is given permission to perform the operations that the grant permits in ARN format. Note that due to eventual consistency issues around IAM principals, the providers's state may not always be refreshed to reflect what is true in AWS.
        #[builder(into)]
        pub grantee_principal: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The unique identifier for the customer master key (CMK) that the grant applies to. Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.
        #[builder(into)]
        pub key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A friendly name for identifying the grant.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of operations that the grant permits. The permitted values are: `Decrypt`, `Encrypt`, `GenerateDataKey`, `GenerateDataKeyWithoutPlaintext`, `ReEncryptFrom`, `ReEncryptTo`, `Sign`, `Verify`, `GetPublicKey`, `CreateGrant`, `RetireGrant`, `DescribeKey`, `GenerateDataKeyPair`, or `GenerateDataKeyPairWithoutPlaintext`.
        #[builder(into)]
        pub operations: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// If set to false (the default) the grants will be revoked upon deletion, and if set to true the grants will try to be retired upon deletion. Note that retiring grants requires special permissions, hence why we default to revoking grants.
        /// See [RetireGrant](https://docs.aws.amazon.com/kms/latest/APIReference/API_RetireGrant.html) for more information.
        #[builder(into, default)]
        pub retire_on_delete: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The principal that is given permission to retire the grant by using RetireGrant operation in ARN format. Note that due to eventual consistency issues around IAM principals, the providers's state may not always be refreshed to reflect what is true in AWS.
        #[builder(into, default)]
        pub retiring_principal: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GrantResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A structure that you can use to allow certain operations in the grant only when the desired encryption context is present. For more information about encryption context, see [Encryption Context](http://docs.aws.amazon.com/kms/latest/developerguide/encryption-context.html).
        pub constraints: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::kms::GrantConstraint>>,
        >,
        /// A list of grant tokens to be used when creating the grant. See [Grant Tokens](http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token) for more information about grant tokens.
        pub grant_creation_tokens: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The unique identifier for the grant.
        pub grant_id: pulumi_gestalt_rust::Output<String>,
        /// The grant token for the created grant. For more information, see [Grant Tokens](http://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token).
        pub grant_token: pulumi_gestalt_rust::Output<String>,
        /// The principal that is given permission to perform the operations that the grant permits in ARN format. Note that due to eventual consistency issues around IAM principals, the providers's state may not always be refreshed to reflect what is true in AWS.
        pub grantee_principal: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the customer master key (CMK) that the grant applies to. Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.
        pub key_id: pulumi_gestalt_rust::Output<String>,
        /// A friendly name for identifying the grant.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of operations that the grant permits. The permitted values are: `Decrypt`, `Encrypt`, `GenerateDataKey`, `GenerateDataKeyWithoutPlaintext`, `ReEncryptFrom`, `ReEncryptTo`, `Sign`, `Verify`, `GetPublicKey`, `CreateGrant`, `RetireGrant`, `DescribeKey`, `GenerateDataKeyPair`, or `GenerateDataKeyPairWithoutPlaintext`.
        pub operations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// If set to false (the default) the grants will be revoked upon deletion, and if set to true the grants will try to be retired upon deletion. Note that retiring grants requires special permissions, hence why we default to revoking grants.
        /// See [RetireGrant](https://docs.aws.amazon.com/kms/latest/APIReference/API_RetireGrant.html) for more information.
        pub retire_on_delete: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The principal that is given permission to retire the grant by using RetireGrant operation in ARN format. Note that due to eventual consistency issues around IAM principals, the providers's state may not always be refreshed to reflect what is true in AWS.
        pub retiring_principal: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GrantArgs,
    ) -> GrantResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let constraints_binding = args.constraints.get_output(context);
        let grant_creation_tokens_binding = args
            .grant_creation_tokens
            .get_output(context);
        let grantee_principal_binding = args.grantee_principal.get_output(context);
        let key_id_binding = args.key_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let operations_binding = args.operations.get_output(context);
        let retire_on_delete_binding = args.retire_on_delete.get_output(context);
        let retiring_principal_binding = args.retiring_principal.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kms/grant:Grant".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "constraints".into(),
                    value: &constraints_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grantCreationTokens".into(),
                    value: &grant_creation_tokens_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "granteePrincipal".into(),
                    value: &grantee_principal_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyId".into(),
                    value: &key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operations".into(),
                    value: &operations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retireOnDelete".into(),
                    value: &retire_on_delete_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retiringPrincipal".into(),
                    value: &retiring_principal_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GrantResult {
            id: o.get_field("id"),
            constraints: o.get_field("constraints"),
            grant_creation_tokens: o.get_field("grantCreationTokens"),
            grant_id: o.get_field("grantId"),
            grant_token: o.get_field("grantToken"),
            grantee_principal: o.get_field("granteePrincipal"),
            key_id: o.get_field("keyId"),
            name: o.get_field("name"),
            operations: o.get_field("operations"),
            retire_on_delete: o.get_field("retireOnDelete"),
            retiring_principal: o.get_field("retiringPrincipal"),
        }
    }
}
