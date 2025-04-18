/// Provides a AWS Clean Rooms collaboration.  All members included in the definition will be invited to
/// join the collaboration and can create memberships.
///
/// ## Example Usage
///
/// ### Collaboration with tags
///
/// ```yaml
/// resources:
///   testCollaboration:
///     type: aws:cleanrooms:Collaboration
///     name: test_collaboration
///     properties:
///       name: pulumi-example-collaboration
///       creatorMemberAbilities:
///         - CAN_QUERY
///         - CAN_RECEIVE_RESULTS
///       creatorDisplayName: 'Creator '
///       description: I made this collaboration with Pulumi!
///       queryLogStatus: DISABLED
///       dataEncryptionMetadata:
///         allowClearText: true
///         allowDuplicates: true
///         allowJoinsOnColumnsWithDifferentNames: true
///         preserveNulls: false
///       members:
///         - accountId: 1.23456789012e+11
///           displayName: Other member
///           memberAbilities: []
///       tags:
///         Project: Pulumi
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_cleanrooms_collaboration` using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cleanrooms/collaboration:Collaboration collaboration 1234abcd-12ab-34cd-56ef-1234567890ab
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod collaboration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CollaborationArgs {
        /// The name for the member record for the collaboration creator.
        #[builder(into)]
        pub creator_display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The list of member abilities for the creator of the collaboration.  Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-creatorMemberAbilities).
        #[builder(into)]
        pub creator_member_abilities: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// a collection of settings which determine how the [c3r client](https://docs.aws.amazon.com/clean-rooms/latest/userguide/crypto-computing.html) will encrypt data for use within this collaboration.
        /// * `data_encryption_metadata.allow_clear_text` - (Required - Forces new resource) - Indicates whether encrypted tables can contain cleartext data. This is a boolea
        /// field.
        /// * `data_encryption_metadata.allow_duplicates` - (Required - Forces new resource ) - Indicates whether Fingerprint columns can contain duplicate entries. This is a
        /// boolean field.
        /// * `data_encryption_metadata.allow_joins_on_columns_with_different_names` - (Required - Forces new resource) - Indicates whether Fingerprint columns can be joined
        /// n any other Fingerprint column with a different name. This is a boolean field.
        /// * `data_encryption_metadata.preserve_nulls` - (Required - Forces new resource) - Indicates whether NULL values are to be copied as NULL to encrypted tables (true)
        /// or cryptographically processed (false).
        #[builder(into, default)]
        pub data_encryption_metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cleanrooms::CollaborationDataEncryptionMetadata>,
        >,
        /// A description for a collaboration.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Additional members of the collaboration which will be invited to join the collaboration.
        /// * `member.account_id` - (Required - Forces new resource) - The account id for the invited member.
        /// * `member.display_name` - (Required - Forces new resource) - The display name for the invited member.
        /// * `member.member_abilities` - (Required - Forces new resource) - The list of abilities for the invited member. Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-creatorMemberAbilities).
        #[builder(into, default)]
        pub members: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cleanrooms::CollaborationMember>>,
        >,
        /// The name of the collaboration.  Collaboration names do not need to be unique.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Determines if members of the collaboration can enable query logs within their own.
        /// emberships. Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-queryLogStatus).
        #[builder(into)]
        pub query_log_status: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key value pairs which tag the collaboration.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CollaborationResult {
        /// The arn of the collaboration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date and time the collaboration was created.
        /// * `member status` - For each member included in the collaboration an additional computed attribute of status is added. These values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_MemberSummary.html#API-Type-MemberSummary-status).
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The name for the member record for the collaboration creator.
        pub creator_display_name: pulumi_gestalt_rust::Output<String>,
        /// The list of member abilities for the creator of the collaboration.  Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-creatorMemberAbilities).
        pub creator_member_abilities: pulumi_gestalt_rust::Output<Vec<String>>,
        /// a collection of settings which determine how the [c3r client](https://docs.aws.amazon.com/clean-rooms/latest/userguide/crypto-computing.html) will encrypt data for use within this collaboration.
        /// * `data_encryption_metadata.allow_clear_text` - (Required - Forces new resource) - Indicates whether encrypted tables can contain cleartext data. This is a boolea
        /// field.
        /// * `data_encryption_metadata.allow_duplicates` - (Required - Forces new resource ) - Indicates whether Fingerprint columns can contain duplicate entries. This is a
        /// boolean field.
        /// * `data_encryption_metadata.allow_joins_on_columns_with_different_names` - (Required - Forces new resource) - Indicates whether Fingerprint columns can be joined
        /// n any other Fingerprint column with a different name. This is a boolean field.
        /// * `data_encryption_metadata.preserve_nulls` - (Required - Forces new resource) - Indicates whether NULL values are to be copied as NULL to encrypted tables (true)
        /// or cryptographically processed (false).
        pub data_encryption_metadata: pulumi_gestalt_rust::Output<
            Option<super::super::types::cleanrooms::CollaborationDataEncryptionMetadata>,
        >,
        /// A description for a collaboration.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Additional members of the collaboration which will be invited to join the collaboration.
        /// * `member.account_id` - (Required - Forces new resource) - The account id for the invited member.
        /// * `member.display_name` - (Required - Forces new resource) - The display name for the invited member.
        /// * `member.member_abilities` - (Required - Forces new resource) - The list of abilities for the invited member. Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-creatorMemberAbilities).
        pub members: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cleanrooms::CollaborationMember>>,
        >,
        /// The name of the collaboration.  Collaboration names do not need to be unique.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Determines if members of the collaboration can enable query logs within their own.
        /// emberships. Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-queryLogStatus).
        pub query_log_status: pulumi_gestalt_rust::Output<String>,
        /// Key value pairs which tag the collaboration.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CollaborationArgs,
    ) -> CollaborationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let creator_display_name_binding = args.creator_display_name.get_output(context);
        let creator_member_abilities_binding = args
            .creator_member_abilities
            .get_output(context);
        let data_encryption_metadata_binding = args
            .data_encryption_metadata
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let members_binding = args.members.get_output(context);
        let name_binding = args.name.get_output(context);
        let query_log_status_binding = args.query_log_status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cleanrooms/collaboration:Collaboration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "creatorDisplayName".into(),
                    value: &creator_display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "creatorMemberAbilities".into(),
                    value: &creator_member_abilities_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataEncryptionMetadata".into(),
                    value: &data_encryption_metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: &members_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryLogStatus".into(),
                    value: &query_log_status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CollaborationResult {
            arn: o.get_field("arn"),
            create_time: o.get_field("createTime"),
            creator_display_name: o.get_field("creatorDisplayName"),
            creator_member_abilities: o.get_field("creatorMemberAbilities"),
            data_encryption_metadata: o.get_field("dataEncryptionMetadata"),
            description: o.get_field("description"),
            members: o.get_field("members"),
            name: o.get_field("name"),
            query_log_status: o.get_field("queryLogStatus"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            update_time: o.get_field("updateTime"),
        }
    }
}
