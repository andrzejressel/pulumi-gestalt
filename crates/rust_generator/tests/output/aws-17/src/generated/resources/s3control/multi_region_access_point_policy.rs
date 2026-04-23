/// Provides a resource to manage an S3 Multi-Region Access Point access control policy.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import Multi-Region Access Point Policies using the `account_id` and `name` of the Multi-Region Access Point separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/multiRegionAccessPointPolicy:MultiRegionAccessPointPolicy example 123456789012:example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod multi_region_access_point_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MultiRegionAccessPointPolicyArgs {
        /// The AWS account ID for the owner of the Multi-Region Access Point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::Input<Option<String>>,
        /// A configuration block containing details about the policy for the Multi-Region Access Point. See Details Configuration Block below for more details
        #[builder(into)]
        pub details: pulumi_gestalt_rust::Input<
            super::super::types::s3control::MultiRegionAccessPointPolicyDetails,
        >,
    }
    #[allow(dead_code)]
    pub struct MultiRegionAccessPointPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID for the owner of the Multi-Region Access Point. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// A configuration block containing details about the policy for the Multi-Region Access Point. See Details Configuration Block below for more details
        pub details: pulumi_gestalt_rust::Output<
            super::super::types::s3control::MultiRegionAccessPointPolicyDetails,
        >,
        /// The last established policy for the Multi-Region Access Point.
        pub established: pulumi_gestalt_rust::Output<String>,
        /// The proposed policy for the Multi-Region Access Point.
        pub proposed: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MultiRegionAccessPointPolicyArgs,
    ) -> MultiRegionAccessPointPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MultiRegionAccessPointPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> MultiRegionAccessPointPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MultiRegionAccessPointPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> MultiRegionAccessPointPolicyResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let details_binding = args.details.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3control/multiRegionAccessPointPolicy:MultiRegionAccessPointPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "details".into(),
                    value: &details_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        MultiRegionAccessPointPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            details: o.get_field("details"),
            established: o.get_field("established"),
            proposed: o.get_field("proposed"),
        }
    }
}
