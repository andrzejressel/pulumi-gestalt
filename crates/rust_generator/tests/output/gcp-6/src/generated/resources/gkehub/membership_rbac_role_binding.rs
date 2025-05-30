/// ## Example Usage
///
/// ### Gkehub Membership Rbac Role Binding Basic
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:Cluster
///     properties:
///       name: basic-cluster
///       location: us-central1-a
///       initialNodeCount: 1
///       deletionProtection: true
///       network: default
///       subnetwork: default
///   membership:
///     type: gcp:gkehub:Membership
///     properties:
///       membershipId: tf-test-membership_75092
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${primary.id}
///     options:
///       dependsOn:
///         - ${primary}
///   membershipRbacRoleBinding:
///     type: gcp:gkehub:MembershipRbacRoleBinding
///     name: membership_rbac_role_binding
///     properties:
///       membershipRbacRoleBindingId: tf-test-membership-rbac-role-binding_2605
///       membershipId: ${membership.membershipId}
///       user: service-${project.number}@gcp-sa-anthossupport.iam.gserviceaccount.com
///       role:
///         predefinedRole: ANTHOS_SUPPORT
///       location: global
///     options:
///       dependsOn:
///         - ${membership}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// MembershipRBACRoleBinding can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/memberships/{{membership_id}}/rbacrolebindings/{{membership_rbac_role_binding_id}}`
///
/// * `{{project}}/{{location}}/{{membership_id}}/{{membership_rbac_role_binding_id}}`
///
/// * `{{location}}/{{membership_id}}/{{membership_rbac_role_binding_id}}`
///
/// When using the `pulumi import` command, MembershipRBACRoleBinding can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkehub/membershipRbacRoleBinding:MembershipRbacRoleBinding default projects/{{project}}/locations/{{location}}/memberships/{{membership_id}}/rbacrolebindings/{{membership_rbac_role_binding_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/membershipRbacRoleBinding:MembershipRbacRoleBinding default {{project}}/{{location}}/{{membership_id}}/{{membership_rbac_role_binding_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/membershipRbacRoleBinding:MembershipRbacRoleBinding default {{location}}/{{membership_id}}/{{membership_rbac_role_binding_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod membership_rbac_role_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MembershipRbacRoleBindingArgs {
        /// Location of the Membership
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Id of the membership
        #[builder(into)]
        pub membership_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The client-provided identifier of the RBAC Role Binding.
        #[builder(into)]
        pub membership_rbac_role_binding_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Role to bind to the principal.
        /// Structure is documented below.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gkehub::MembershipRbacRoleBindingRole,
        >,
        /// Principal that is be authorized in the cluster (at least of one the oneof
        /// is required). Updating one will unset the other automatically.
        /// user is the name of the user as seen by the kubernetes cluster, example
        /// "alice" or "alice@domain.tld"
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MembershipRbacRoleBindingResult {
        /// Time the RBAC Role Binding was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Time the RBAC Role Binding was deleted in UTC.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// Location of the Membership
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Id of the membership
        pub membership_id: pulumi_gestalt_rust::Output<String>,
        /// The client-provided identifier of the RBAC Role Binding.
        pub membership_rbac_role_binding_id: pulumi_gestalt_rust::Output<String>,
        /// The resource name for the RBAC Role Binding
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Role to bind to the principal.
        /// Structure is documented below.
        pub role: pulumi_gestalt_rust::Output<
            super::super::types::gkehub::MembershipRbacRoleBindingRole,
        >,
        /// State of the RBAC Role Binding resource.
        /// Structure is documented below.
        pub states: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkehub::MembershipRbacRoleBindingState>,
        >,
        /// Google-generated UUID for this resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Time the RBAC Role Binding was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Principal that is be authorized in the cluster (at least of one the oneof
        /// is required). Updating one will unset the other automatically.
        /// user is the name of the user as seen by the kubernetes cluster, example
        /// "alice" or "alice@domain.tld"
        pub user: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MembershipRbacRoleBindingArgs,
    ) -> MembershipRbacRoleBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let membership_id_binding = args.membership_id.get_output(context);
        let membership_rbac_role_binding_id_binding = args
            .membership_rbac_role_binding_id
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let role_binding = args.role.get_output(context);
        let user_binding = args.user.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gkehub/membershipRbacRoleBinding:MembershipRbacRoleBinding"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "membershipId".into(),
                    value: &membership_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "membershipRbacRoleBindingId".into(),
                    value: &membership_rbac_role_binding_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: &user_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MembershipRbacRoleBindingResult {
            create_time: o.get_field("createTime"),
            delete_time: o.get_field("deleteTime"),
            location: o.get_field("location"),
            membership_id: o.get_field("membershipId"),
            membership_rbac_role_binding_id: o.get_field("membershipRbacRoleBindingId"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            role: o.get_field("role"),
            states: o.get_field("states"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            user: o.get_field("user"),
        }
    }
}
