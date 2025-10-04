/// Three different resources help you manage your IAM policy for Compute Engine Subnetwork. Each of these resources serves a different use case:
///
/// * `gcp.compute.SubnetworkIAMPolicy`: Authoritative. Sets the IAM policy for the subnetwork and replaces any existing policy already attached.
/// * `gcp.compute.SubnetworkIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the subnetwork are preserved.
/// * `gcp.compute.SubnetworkIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the subnetwork are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.compute.SubnetworkIAMPolicy`: Retrieves the IAM policy for the subnetwork
///
/// > **Note:** `gcp.compute.SubnetworkIAMPolicy` **cannot** be used in conjunction with `gcp.compute.SubnetworkIAMBinding` and `gcp.compute.SubnetworkIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.compute.SubnetworkIAMBinding` resources **can be** used in conjunction with `gcp.compute.SubnetworkIAMMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.compute.SubnetworkIAMPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:SubnetworkIAMPolicy
///     properties:
///       project: ${["network-with-private-secondary-ip-ranges"].project}
///       region: ${["network-with-private-secondary-ip-ranges"].region}
///       subnetwork: ${["network-with-private-secondary-ip-ranges"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/compute.networkUser
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:SubnetworkIAMPolicy
///     properties:
///       project: ${["network-with-private-secondary-ip-ranges"].project}
///       region: ${["network-with-private-secondary-ip-ranges"].region}
///       subnetwork: ${["network-with-private-secondary-ip-ranges"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/compute.networkUser
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.compute.SubnetworkIAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = subnetwork_iam_binding::create(
///         "binding",
///         SubnetworkIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"network-with-private-secondary-ip-ranges\"].project}")
///             .region("${[\"network-with-private-secondary-ip-ranges\"].region}")
///             .role("roles/compute.networkUser")
///             .subnetwork("${[\"network-with-private-secondary-ip-ranges\"].name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = subnetwork_iam_binding::create(
///         "binding",
///         SubnetworkIamBindingArgs::builder()
///             .condition(
///                 SubnetworkIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"network-with-private-secondary-ip-ranges\"].project}")
///             .region("${[\"network-with-private-secondary-ip-ranges\"].region}")
///             .role("roles/compute.networkUser")
///             .subnetwork("${[\"network-with-private-secondary-ip-ranges\"].name}")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.compute.SubnetworkIAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = subnetwork_iam_member::create(
///         "member",
///         SubnetworkIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${[\"network-with-private-secondary-ip-ranges\"].project}")
///             .region("${[\"network-with-private-secondary-ip-ranges\"].region}")
///             .role("roles/compute.networkUser")
///             .subnetwork("${[\"network-with-private-secondary-ip-ranges\"].name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = subnetwork_iam_member::create(
///         "member",
///         SubnetworkIamMemberArgs::builder()
///             .condition(
///                 SubnetworkIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("${[\"network-with-private-secondary-ip-ranges\"].project}")
///             .region("${[\"network-with-private-secondary-ip-ranges\"].region}")
///             .role("roles/compute.networkUser")
///             .subnetwork("${[\"network-with-private-secondary-ip-ranges\"].name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Compute Engine Subnetwork
/// Three different resources help you manage your IAM policy for Compute Engine Subnetwork. Each of these resources serves a different use case:
///
/// * `gcp.compute.SubnetworkIAMPolicy`: Authoritative. Sets the IAM policy for the subnetwork and replaces any existing policy already attached.
/// * `gcp.compute.SubnetworkIAMBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the subnetwork are preserved.
/// * `gcp.compute.SubnetworkIAMMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the subnetwork are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.compute.SubnetworkIAMPolicy`: Retrieves the IAM policy for the subnetwork
///
/// > **Note:** `gcp.compute.SubnetworkIAMPolicy` **cannot** be used in conjunction with `gcp.compute.SubnetworkIAMBinding` and `gcp.compute.SubnetworkIAMMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.compute.SubnetworkIAMBinding` resources **can be** used in conjunction with `gcp.compute.SubnetworkIAMMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.compute.SubnetworkIAMPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:SubnetworkIAMPolicy
///     properties:
///       project: ${["network-with-private-secondary-ip-ranges"].project}
///       region: ${["network-with-private-secondary-ip-ranges"].region}
///       subnetwork: ${["network-with-private-secondary-ip-ranges"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/compute.networkUser
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:compute:SubnetworkIAMPolicy
///     properties:
///       project: ${["network-with-private-secondary-ip-ranges"].project}
///       region: ${["network-with-private-secondary-ip-ranges"].region}
///       subnetwork: ${["network-with-private-secondary-ip-ranges"].name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/compute.networkUser
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.compute.SubnetworkIAMBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = subnetwork_iam_binding::create(
///         "binding",
///         SubnetworkIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"network-with-private-secondary-ip-ranges\"].project}")
///             .region("${[\"network-with-private-secondary-ip-ranges\"].region}")
///             .role("roles/compute.networkUser")
///             .subnetwork("${[\"network-with-private-secondary-ip-ranges\"].name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = subnetwork_iam_binding::create(
///         "binding",
///         SubnetworkIamBindingArgs::builder()
///             .condition(
///                 SubnetworkIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("${[\"network-with-private-secondary-ip-ranges\"].project}")
///             .region("${[\"network-with-private-secondary-ip-ranges\"].region}")
///             .role("roles/compute.networkUser")
///             .subnetwork("${[\"network-with-private-secondary-ip-ranges\"].name}")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.compute.SubnetworkIAMMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = subnetwork_iam_member::create(
///         "member",
///         SubnetworkIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .project("${[\"network-with-private-secondary-ip-ranges\"].project}")
///             .region("${[\"network-with-private-secondary-ip-ranges\"].region}")
///             .role("roles/compute.networkUser")
///             .subnetwork("${[\"network-with-private-secondary-ip-ranges\"].name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = subnetwork_iam_member::create(
///         "member",
///         SubnetworkIamMemberArgs::builder()
///             .condition(
///                 SubnetworkIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("${[\"network-with-private-secondary-ip-ranges\"].project}")
///             .region("${[\"network-with-private-secondary-ip-ranges\"].region}")
///             .role("roles/compute.networkUser")
///             .subnetwork("${[\"network-with-private-secondary-ip-ranges\"].name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/regions/{{region}}/subnetworks/{{name}}
///
/// * {{project}}/{{region}}/{{name}}
///
/// * {{region}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Compute Engine subnetwork IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/subnetworkIAMPolicy:SubnetworkIAMPolicy editor "projects/{{project}}/regions/{{region}}/subnetworks/{{subnetwork}} roles/compute.networkUser user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/subnetworkIAMPolicy:SubnetworkIAMPolicy editor "projects/{{project}}/regions/{{region}}/subnetworks/{{subnetwork}} roles/compute.networkUser"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:compute/subnetworkIAMPolicy:SubnetworkIAMPolicy editor projects/{{project}}/regions/{{region}}/subnetworks/{{subnetwork}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subnetwork_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetworkIAMPolicyArgs {
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The GCP region for this subnetwork.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no region is provided in the parent identifier and no
        /// region is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub subnetwork: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetworkIAMPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The GCP region for this subnetwork.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no region is provided in the parent identifier and no
        /// region is specified, it is taken from the provider configuration.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub subnetwork: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubnetworkIAMPolicyArgs,
    ) -> SubnetworkIAMPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let subnetwork_binding = args.subnetwork.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/subnetworkIAMPolicy:SubnetworkIAMPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubnetworkIAMPolicyResult {
            id: o.get_field("id"),
            etag: o.get_field("etag"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            subnetwork: o.get_field("subnetwork"),
        }
    }
}
