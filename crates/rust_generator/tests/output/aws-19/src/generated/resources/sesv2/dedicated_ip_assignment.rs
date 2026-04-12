/// Resource for managing an AWS SESv2 (Simple Email V2) Dedicated IP Assignment.
///
/// This resource is used with "Standard" dedicated IP addresses. This includes addresses [requested and relinquished manually](https://docs.aws.amazon.com/ses/latest/dg/dedicated-ip-case.html) via an AWS support case, or [Bring Your Own IP](https://docs.aws.amazon.com/ses/latest/dg/dedicated-ip-byo.html) addresses. Once no longer assigned, this resource returns the IP to the [`ses-default-dedicated-pool`](https://docs.aws.amazon.com/ses/latest/dg/managing-ip-pools.html), managed by AWS.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = dedicated_ip_assignment::create(
///         "example",
///         DedicatedIpAssignmentArgs::builder()
///             .destination_pool_name("my-pool")
///             .ip("0.0.0.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SESv2 (Simple Email V2) Dedicated IP Assignment using the `id`, which is a comma-separated string made up of `ip` and `destination_pool_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sesv2/dedicatedIpAssignment:DedicatedIpAssignment example "0.0.0.0,my-pool"
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod dedicated_ip_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DedicatedIpAssignmentArgs {
        /// Dedicated IP address.
        #[builder(into)]
        pub destination_pool_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Dedicated IP address.
        #[builder(into)]
        pub ip: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DedicatedIpAssignmentResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Dedicated IP address.
        pub destination_pool_name: pulumi_gestalt_rust::Output<String>,
        /// Dedicated IP address.
        pub ip: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DedicatedIpAssignmentArgs,
    ) -> DedicatedIpAssignmentResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DedicatedIpAssignmentArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DedicatedIpAssignmentResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DedicatedIpAssignmentArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DedicatedIpAssignmentResult {
        let destination_pool_name_binding = args.destination_pool_name.get_output(ctx);
        let ip_binding = args.ip.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sesv2/dedicatedIpAssignment:DedicatedIpAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationPoolName".into(),
                    value: &destination_pool_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ip".into(),
                    value: &ip_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DedicatedIpAssignmentResult {
            id: o.get_id(),
            urn: o.get_urn(),
            destination_pool_name: o.get_field("destinationPoolName"),
            ip: o.get_field("ip"),
        }
    }
}
