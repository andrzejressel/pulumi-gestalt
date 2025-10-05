/// A Cloud Identity resource representing a Group.
///
///
/// To get more information about Group, see:
///
/// * [API documentation](https://cloud.google.com/identity/docs/reference/rest/v1beta1/groups)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/identity/docs/how-to/setup)
///
/// > **Warning:** If you are using User ADCs (Application Default Credentials) with this resource,
/// you must specify a `billing_project` and set `user_project_override` to true
/// in the provider configuration. Otherwise the Cloud Identity API will return a 403 error.
/// Your account must have the `serviceusage.services.use` permission on the
/// `billing_project` you defined.
///
/// ## Example Usage
///
/// ### Cloud Identity Groups Basic
///
///
/// ```yaml
/// resources:
///   cloudIdentityGroupBasic:
///     type: gcp:cloudidentity:Group
///     name: cloud_identity_group_basic
///     properties:
///       displayName: my-identity-group
///       initialGroupConfig: WITH_INITIAL_OWNER
///       parent: customers/A01b123xz
///       groupKey:
///         id: my-identity-group@example.com
///       labels:
///         cloudidentity.googleapis.com/groups.discussion_forum: ""
/// ```
///
/// ## Import
///
/// Group can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Group can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudidentity/group:Group default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// An extended description to help users determine the purpose of a Group. Must not be longer than 4,096 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the Group.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// EntityKey of the Group.
        /// Structure is documented below.
        #[builder(into)]
        pub group_key: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudidentity::GroupGroupKey,
        >,
        /// The initial configuration options for creating a Group. See the [API
        /// reference](https://cloud.google.com/identity/docs/reference/rest/v1beta1/groups/create#initialgroupconfig) for possible
        /// values. Default value: "EMPTY" Possible values: ["INITIAL_GROUP_CONFIG_UNSPECIFIED", "WITH_INITIAL_OWNER", "EMPTY"]
        #[builder(into, default)]
        pub initial_group_config: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more label entries that apply to the Group. Currently supported labels contain a key with an empty value.
        /// Google Groups are the default type of group and have a label with a key of cloudidentity.googleapis.com/groups.discussion_forum and an empty value.
        /// Existing Google Groups can have an additional label with a key of cloudidentity.googleapis.com/groups.security and an empty value added to them. This is an immutable change and the security label cannot be removed once added.
        /// Dynamic groups have a label with a key of cloudidentity.googleapis.com/groups.dynamic.
        /// Identity-mapped groups for Cloud Search have a label with a key of system/groups/external and an empty value.
        #[builder(into)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
        /// The resource name of the entity under which this Group resides in the
        /// Cloud Identity resource hierarchy.
        /// Must be of the form identitysources/{identity_source_id} for external-identity-mapped
        /// groups or customers/{customer_id} for Google Groups.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Additional group keys associated with the Group
        /// Structure is documented below.
        pub additional_group_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudidentity::GroupAdditionalGroupKey>,
        >,
        /// The time when the Group was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// An extended description to help users determine the purpose of a Group. Must not be longer than 4,096 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of the Group.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// EntityKey of the Group.
        /// Structure is documented below.
        pub group_key: pulumi_gestalt_rust::Output<
            super::super::types::cloudidentity::GroupGroupKey,
        >,
        /// The initial configuration options for creating a Group. See the [API
        /// reference](https://cloud.google.com/identity/docs/reference/rest/v1beta1/groups/create#initialgroupconfig) for possible
        /// values. Default value: "EMPTY" Possible values: ["INITIAL_GROUP_CONFIG_UNSPECIFIED", "WITH_INITIAL_OWNER", "EMPTY"]
        pub initial_group_config: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more label entries that apply to the Group. Currently supported labels contain a key with an empty value.
        /// Google Groups are the default type of group and have a label with a key of cloudidentity.googleapis.com/groups.discussion_forum and an empty value.
        /// Existing Google Groups can have an additional label with a key of cloudidentity.googleapis.com/groups.security and an empty value added to them. This is an immutable change and the security label cannot be removed once added.
        /// Dynamic groups have a label with a key of cloudidentity.googleapis.com/groups.dynamic.
        /// Identity-mapped groups for Cloud Search have a label with a key of system/groups/external and an empty value.
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource name of the Group in the format: groups/{group_id}, where group_id
        /// is the unique ID assigned to the Group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the entity under which this Group resides in the
        /// Cloud Identity resource hierarchy.
        /// Must be of the form identitysources/{identity_source_id} for external-identity-mapped
        /// groups or customers/{customer_id} for Google Groups.
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// The time when the Group was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupArgs,
    ) -> GroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let group_key_binding = args.group_key.get_output(context);
        let initial_group_config_binding = args.initial_group_config.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudidentity/group:Group".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupKey".into(),
                    value: &group_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initialGroupConfig".into(),
                    value: &initial_group_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupResult {
            id: o.get_field("id"),
            additional_group_keys: o.get_field("additionalGroupKeys"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            group_key: o.get_field("groupKey"),
            initial_group_config: o.get_field("initialGroupConfig"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            update_time: o.get_field("updateTime"),
        }
    }
}
