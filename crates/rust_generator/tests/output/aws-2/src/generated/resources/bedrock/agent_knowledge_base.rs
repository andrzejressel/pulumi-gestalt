/// Resource for managing an AWS Agents for Amazon Bedrock Knowledge Base.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:bedrock:AgentKnowledgeBase
///     properties:
///       name: example
///       roleArn: ${exampleAwsIamRole.arn}
///       knowledgeBaseConfiguration:
///         vectorKnowledgeBaseConfiguration:
///           embeddingModelArn: arn:aws:bedrock:us-west-2::foundation-model/amazon.titan-embed-text-v1
///         type: VECTOR
///       storageConfiguration:
///         type: OPENSEARCH_SERVERLESS
///         opensearchServerlessConfiguration:
///           collectionArn: arn:aws:aoss:us-west-2:123456789012:collection/142bezjddq707i5stcrf
///           vectorIndexName: bedrock-knowledge-base-default-index
///           fieldMapping:
///             vectorField: bedrock-knowledge-base-default-vector
///             textField: AMAZON_BEDROCK_TEXT_CHUNK
///             metadataField: AMAZON_BEDROCK_METADATA
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Agents for Amazon Bedrock Knowledge Base using the knowledge base ID. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/agentKnowledgeBase:AgentKnowledgeBase example EMDPPAYPZI
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod agent_knowledge_base {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgentKnowledgeBaseArgs {
        /// Description of the knowledge base.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Details about the embeddings configuration of the knowledge base. See `knowledge_base_configuration` block for details.
        #[builder(into, default)]
        pub knowledge_base_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::bedrock::AgentKnowledgeBaseKnowledgeBaseConfiguration,
            >,
        >,
        /// Name of the knowledge base.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the IAM role with permissions to invoke API operations on the knowledge base.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Details about the storage configuration of the knowledge base. See `storage_configuration` block for details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub storage_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfiguration>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::AgentKnowledgeBaseTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct AgentKnowledgeBaseResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the knowledge base.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Time at which the knowledge base was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Description of the knowledge base.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub failure_reasons: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Details about the embeddings configuration of the knowledge base. See `knowledge_base_configuration` block for details.
        pub knowledge_base_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::bedrock::AgentKnowledgeBaseKnowledgeBaseConfiguration,
            >,
        >,
        /// Name of the knowledge base.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ARN of the IAM role with permissions to invoke API operations on the knowledge base.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Details about the storage configuration of the knowledge base. See `storage_configuration` block for details.
        ///
        /// The following arguments are optional:
        pub storage_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfiguration>,
        >,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::AgentKnowledgeBaseTimeouts>,
        >,
        /// Time at which the knowledge base was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AgentKnowledgeBaseArgs,
    ) -> AgentKnowledgeBaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let knowledge_base_configuration_binding = args
            .knowledge_base_configuration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let storage_configuration_binding = args
            .storage_configuration
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:bedrock/agentKnowledgeBase:AgentKnowledgeBase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "knowledgeBaseConfiguration".into(),
                    value: &knowledge_base_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageConfiguration".into(),
                    value: &storage_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AgentKnowledgeBaseResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            description: o.get_field("description"),
            failure_reasons: o.get_field("failureReasons"),
            knowledge_base_configuration: o.get_field("knowledgeBaseConfiguration"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
            storage_configuration: o.get_field("storageConfiguration"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            updated_at: o.get_field("updatedAt"),
        }
    }
}
