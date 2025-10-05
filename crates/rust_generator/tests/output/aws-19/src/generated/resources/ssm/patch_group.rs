/// Provides an SSM Patch Group resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let patchgroup = patch_group::create(
///         "patchgroup",
///         PatchGroupArgs::builder()
///             .baseline_id("${production.id}")
///             .patch_group("patch-group-name")
///             .build_struct(),
///     );
///     let production = patch_baseline::create(
///         "production",
///         PatchBaselineArgs::builder()
///             .approved_patches(vec!["KB123456",])
///             .name("patch-baseline")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod patch_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PatchGroupArgs {
        /// The ID of the patch baseline to register the patch group with.
        #[builder(into)]
        pub baseline_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the patch group that should be registered with the patch baseline.
        #[builder(into)]
        pub patch_group: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PatchGroupResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the patch baseline to register the patch group with.
        pub baseline_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the patch group that should be registered with the patch baseline.
        pub patch_group: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PatchGroupArgs,
    ) -> PatchGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let baseline_id_binding = args.baseline_id.get_output(context);
        let patch_group_binding = args.patch_group.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssm/patchGroup:PatchGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baselineId".into(),
                    value: &baseline_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "patchGroup".into(),
                    value: &patch_group_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PatchGroupResult {
            id: o.get_field("id"),
            baseline_id: o.get_field("baselineId"),
            patch_group: o.get_field("patchGroup"),
        }
    }
}
