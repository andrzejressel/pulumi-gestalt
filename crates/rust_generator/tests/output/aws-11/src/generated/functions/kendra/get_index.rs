#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_index {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIndexArgs {
        /// Returns information on a specific Index by id.
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata that helps organize the Indices you create.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetIndexResult {
        /// ARN of the Index.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Block that sets the number of additional document storage and query capacity units that should be used by the index. Documented below.
        pub capacity_units: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kendra::GetIndexCapacityUnit>,
        >,
        /// Unix datetime that the index was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Description of the Index.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// One or more blocks that specify the configuration settings for any metadata applied to the documents in the index. Documented below.
        pub document_metadata_configuration_updates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::kendra::GetIndexDocumentMetadataConfigurationUpdate,
            >,
        >,
        /// Amazon Kendra edition for the index.
        pub edition: pulumi_gestalt_rust::Output<String>,
        /// When the Status field value is `FAILED`, this contains a message that explains why.
        pub error_message: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the Index.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Block that provides information about the number of FAQ questions and answers and the number of text documents indexed. Documented below.
        pub index_statistics: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kendra::GetIndexIndexStatistic>,
        >,
        /// Name of the index field. Minimum length of 1. Maximum length of 30.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An AWS Identity and Access Management (IAM) role that gives Amazon Kendra permissions to access your Amazon CloudWatch logs and metrics. This is also the role you use when you call the `BatchPutDocument` API to index documents from an Amazon S3 bucket.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// A block that specifies the identifier of the AWS KMS customer managed key (CMK) that's used to encrypt data indexed by Amazon Kendra. Amazon Kendra doesn't support asymmetric CMKs. Documented below.
        pub server_side_encryption_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::kendra::GetIndexServerSideEncryptionConfiguration,
            >,
        >,
        /// Current status of the index. When the value is `ACTIVE`, the index is ready for use. If the Status field value is `FAILED`, the `error_message` field contains a message that explains why.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Metadata that helps organize the Indices you create.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Unix datetime that the index was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
        /// User context policy. Valid values are `ATTRIBUTE_FILTER` or `USER_TOKEN`. For more information, refer to [UserContextPolicy](https://docs.aws.amazon.com/kendra/latest/APIReference/API_CreateIndex.html#kendra-CreateIndex-request-UserContextPolicy).
        pub user_context_policy: pulumi_gestalt_rust::Output<String>,
        /// A block that enables fetching access levels of groups and users from an AWS Single Sign-On identity source. Documented below.
        pub user_group_resolution_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::kendra::GetIndexUserGroupResolutionConfiguration,
            >,
        >,
        /// A block that specifies the user token configuration. Documented below.
        pub user_token_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::kendra::GetIndexUserTokenConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetIndexArgs,
    ) -> GetIndexResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:kendra/getIndex:getIndex".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetIndexResult {
            arn: o.get_field("arn"),
            capacity_units: o.get_field("capacityUnits"),
            created_at: o.get_field("createdAt"),
            description: o.get_field("description"),
            document_metadata_configuration_updates: o
                .get_field("documentMetadataConfigurationUpdates"),
            edition: o.get_field("edition"),
            error_message: o.get_field("errorMessage"),
            id: o.get_field("id"),
            index_statistics: o.get_field("indexStatistics"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
            server_side_encryption_configurations: o
                .get_field("serverSideEncryptionConfigurations"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            updated_at: o.get_field("updatedAt"),
            user_context_policy: o.get_field("userContextPolicy"),
            user_group_resolution_configurations: o
                .get_field("userGroupResolutionConfigurations"),
            user_token_configurations: o.get_field("userTokenConfigurations"),
        }
    }
}
