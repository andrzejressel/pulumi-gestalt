/// Provides a Managed Scaling policy for EMR Cluster. With Amazon EMR versions 5.30.0 and later (except for Amazon EMR 6.0.0), you can enable EMR managed scaling to automatically increase or decrease the number of instances or units in your cluster based on workload. See [Using EMR Managed Scaling in Amazon EMR](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-managed-scaling.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sample = cluster::create(
///         "sample",
///         ClusterArgs::builder()
///             .core_instance_group(
///                 ClusterCoreInstanceGroup::builder()
///                     .instanceType("c4.large")
///                     .build_struct(),
///             )
///             .master_instance_group(
///                 ClusterMasterInstanceGroup::builder()
///                     .instanceType("m4.large")
///                     .build_struct(),
///             )
///             .name("emr-sample-cluster")
///             .release_label("emr-5.30.0")
///             .build_struct(),
///     );
///     let samplepolicy = managed_scaling_policy::create(
///         "samplepolicy",
///         ManagedScalingPolicyArgs::builder()
///             .cluster_id("${sample.id}")
///             .compute_limits(
///                 vec![
///                     ManagedScalingPolicyComputeLimit::builder().maximumCapacityUnits(10)
///                     .maximumCoreCapacityUnits(10).maximumOndemandCapacityUnits(2)
///                     .minimumCapacityUnits(2).unitType("Instances").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EMR Managed Scaling Policies using the EMR Cluster identifier. For example:
///
/// ```sh
/// $ pulumi import aws:emr/managedScalingPolicy:ManagedScalingPolicy example j-123456ABCDEF
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod managed_scaling_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedScalingPolicyArgs {
        /// ID of the EMR cluster
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::Input<String>,
        /// Configuration block with compute limit settings. Described below.
        #[builder(into)]
        pub compute_limits: pulumi_gestalt_rust::Input<
            Vec<super::super::types::emr::ManagedScalingPolicyComputeLimit>,
        >,
    }
    #[allow(dead_code)]
    pub struct ManagedScalingPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// ID of the EMR cluster
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with compute limit settings. Described below.
        pub compute_limits: pulumi_gestalt_rust::Output<
            Vec<super::super::types::emr::ManagedScalingPolicyComputeLimit>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedScalingPolicyArgs,
    ) -> ManagedScalingPolicyResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedScalingPolicyArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ManagedScalingPolicyResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedScalingPolicyArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ManagedScalingPolicyResult {
        let cluster_id_binding = args.cluster_id.get_output(ctx);
        let compute_limits_binding = args.compute_limits.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:emr/managedScalingPolicy:ManagedScalingPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computeLimits".into(),
                    value: &compute_limits_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ManagedScalingPolicyResult {
            id: o.get_id(),
            urn: o.get_urn(),
            cluster_id: o.get_field("clusterId"),
            compute_limits: o.get_field("computeLimits"),
        }
    }
}
