/// Allows management of Organization Policies for a Google Cloud Organization.
///
/// > **Warning:** This resource has been superseded by `gcp.orgpolicy.Policy`. `gcp.orgpolicy.Policy` uses Organization Policy API V2 instead of Cloud Resource Manager API V1 and it supports additional features such as tags and conditions.
///
/// To get more information about Organization Policies, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/reference/rest/v1/organizations/setOrgPolicy)
/// * How-to Guides
///     * [Introduction to the Organization Policy Service](https://cloud.google.com/resource-manager/docs/organization-policy/overview)
///
/// ## Example Usage
///
/// To set policy with a [boolean constraint](https://cloud.google.com/resource-manager/docs/organization-policy/quickstart-boolean-constraints):
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let serialPortPolicy = policy::create(
///         "serialPortPolicy",
///         PolicyArgs::builder()
///             .boolean_policy(PolicyBooleanPolicy::builder().enforced(true).build_struct())
///             .constraint("compute.disableSerialPortAccess")
///             .org_id("123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// To set a policy with a [list constraint](https://cloud.google.com/resource-manager/docs/organization-policy/quickstart-list-constraints):
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let servicesPolicy = policy::create(
///         "servicesPolicy",
///         PolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .list_policy(
///                 PolicyListPolicy::builder()
///                     .allow(PolicyListPolicyAllow::builder().all(true).build_struct())
///                     .build_struct(),
///             )
///             .org_id("123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Or to deny some services, use the following instead:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let servicesPolicy = policy::create(
///         "servicesPolicy",
///         PolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .list_policy(
///                 PolicyListPolicy::builder()
///                     .deny(
///                         PolicyListPolicyDeny::builder()
///                             .values(vec!["cloudresourcemanager.googleapis.com",])
///                             .build_struct(),
///                     )
///                     .suggestedValue("compute.googleapis.com")
///                     .build_struct(),
///             )
///             .org_id("123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// To restore the default organization policy, use the following instead:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let servicesPolicy = policy::create(
///         "servicesPolicy",
///         PolicyArgs::builder()
///             .constraint("serviceuser.services")
///             .org_id("123456789")
///             .restore_policy(PolicyRestorePolicy::builder().default(true).build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Organization Policies can be imported using the `org_id` and the `constraint`, e.g.
///
/// * `{{org_id}}/constraints/{{constraint}}`
///
/// When using the `pulumi import` command, Organization Policies can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:organizations/policy:Policy default {{org_id}}/constraints/{{constraint}}
/// ```
///
/// It is all right if the constraint contains a slash, as in the example above.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// A boolean policy is a constraint that is either enforced or not. Structure is documented
        /// below.
        #[builder(into, default)]
        pub boolean_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::organizations::PolicyBooleanPolicy>,
        >,
        /// The name of the Constraint the Policy is configuring, for example, `serviceuser.services`. Check out the [complete list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-constraints#available_constraints).
        ///
        /// - - -
        #[builder(into)]
        pub constraint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. Structure is documented below.
        #[builder(into, default)]
        pub list_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::organizations::PolicyListPolicy>,
        >,
        /// The numeric ID of the organization to set the policy for.
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A restore policy is a constraint to restore the default policy. Structure is documented below.
        ///
        /// > **Note:** If none of [`boolean_policy`, `list_policy`, `restore_policy`] are defined the policy for a given constraint will
        /// effectively be unset. This is represented in the UI as the constraint being 'Inherited'.
        ///
        /// - - -
        #[builder(into, default)]
        pub restore_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::organizations::PolicyRestorePolicy>,
        >,
        /// Version of the Policy. Default version is 0.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// A boolean policy is a constraint that is either enforced or not. Structure is documented
        /// below.
        pub boolean_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::organizations::PolicyBooleanPolicy>,
        >,
        /// The name of the Constraint the Policy is configuring, for example, `serviceuser.services`. Check out the [complete list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-constraints#available_constraints).
        ///
        /// - - -
        pub constraint: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the organization policy. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// A policy that can define specific values that are allowed or denied for the given constraint. It can also be used to allow or deny all values. Structure is documented below.
        pub list_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::organizations::PolicyListPolicy>,
        >,
        /// The numeric ID of the organization to set the policy for.
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// A restore policy is a constraint to restore the default policy. Structure is documented below.
        ///
        /// > **Note:** If none of [`boolean_policy`, `list_policy`, `restore_policy`] are defined the policy for a given constraint will
        /// effectively be unset. This is represented in the UI as the constraint being 'Inherited'.
        ///
        /// - - -
        pub restore_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::organizations::PolicyRestorePolicy>,
        >,
        /// (Computed) The timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds, representing when the variable was last updated. Example: "2016-10-09T12:33:37.578138407Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Version of the Policy. Default version is 0.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let boolean_policy_binding = args.boolean_policy.get_output(context);
        let constraint_binding = args.constraint.get_output(context);
        let list_policy_binding = args.list_policy.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let restore_policy_binding = args.restore_policy.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:organizations/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "booleanPolicy".into(),
                    value: &boolean_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "constraint".into(),
                    value: &constraint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listPolicy".into(),
                    value: &list_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restorePolicy".into(),
                    value: &restore_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyResult {
            boolean_policy: o.get_field("booleanPolicy"),
            constraint: o.get_field("constraint"),
            etag: o.get_field("etag"),
            list_policy: o.get_field("listPolicy"),
            org_id: o.get_field("orgId"),
            restore_policy: o.get_field("restorePolicy"),
            update_time: o.get_field("updateTime"),
            version: o.get_field("version"),
        }
    }
}
