#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_policies_for_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPoliciesForTargetArgs {
        /// Must supply one of the 5 different policy filters for a target (AISERVICES_OPT_OUT_POLICY | BACKUP_POLICY | RESOURCE_CONTROL_POLICY | SERVICE_CONTROL_POLICY | TAG_POLICY)
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The root (string that begins with "r-" followed by 4-32 lowercase letters or digits), account (12 digit string), or Organizational Unit (string starting with "ou-" followed by 4-32 lowercase letters or digits. This string is followed by a second "-" dash and from 8-32 additional lowercase letters or digits.)
        #[builder(into)]
        pub target_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPoliciesForTargetResult {
        pub filter: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of all the policy ids found.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub target_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPoliciesForTargetArgs,
    ) -> GetPoliciesForTargetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filter_binding = args.filter.get_output(context);
        let target_id_binding = args.target_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:organizations/getPoliciesForTarget:getPoliciesForTarget".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetId".into(),
                    value: &target_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPoliciesForTargetResult {
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            ids: o.get_field("ids"),
            target_id: o.get_field("targetId"),
        }
    }
}
