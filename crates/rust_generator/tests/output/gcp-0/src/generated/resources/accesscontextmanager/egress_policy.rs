/// This resource has been deprecated, please refer to ServicePerimeterEgressPolicy.
///
///
/// To get more information about EgressPolicy, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.servicePerimeters#egresspolicy)
///
/// ## Import
///
/// EgressPolicy can be imported using any of these accepted formats:
///
/// * `{{egress_policy_name}}/{{resource}}`
///
/// When using the `pulumi import` command, EgressPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/egressPolicy:EgressPolicy default {{egress_policy_name}}/{{resource}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod egress_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EgressPolicyArgs {
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub egress_policy_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A GCP resource that is inside of the service perimeter.
        #[builder(into)]
        pub resource: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EgressPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        pub egress_policy_name: pulumi_gestalt_rust::Output<String>,
        /// A GCP resource that is inside of the service perimeter.
        pub resource: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EgressPolicyArgs,
    ) -> EgressPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EgressPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> EgressPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EgressPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> EgressPolicyResult {
        let egress_policy_name_binding = args.egress_policy_name.get_output(ctx);
        let resource_binding = args.resource.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/egressPolicy:EgressPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "egressPolicyName".into(),
                    value: &egress_policy_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resource".into(),
                    value: &resource_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        EgressPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            egress_policy_name: o.get_field("egressPolicyName"),
            resource: o.get_field("resource"),
        }
    }
}
