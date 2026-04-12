/// Starts and stops asynchronous persistent disk replication. For more information
/// see [the official documentation](https://cloud.google.com/compute/docs/disks/async-pd/about)
/// and the [API](https://cloud.google.com/compute/docs/reference/rest/v1/disks).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   primary-disk:
///     type: gcp:compute:Disk
///     properties:
///       name: primary-disk
///       type: pd-ssd
///       zone: europe-west4-a
///       physicalBlockSizeBytes: 4096
///   secondary-disk:
///     type: gcp:compute:Disk
///     properties:
///       name: secondary-disk
///       type: pd-ssd
///       zone: europe-west3-a
///       asyncPrimaryDisk:
///         disk: ${["primary-disk"].id}
///       physicalBlockSizeBytes: 4096
///   replication:
///     type: gcp:compute:DiskAsyncReplication
///     properties:
///       primaryDisk: ${["primary-disk"].id}
///       secondaryDisk:
///         disk: ${["secondary-disk"].id}
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod disk_async_replication {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiskAsyncReplicationArgs {
        /// The primary disk (source of replication).
        #[builder(into)]
        pub primary_disk: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The secondary disk (target of replication). You can specify only one value. Structure is documented below.
        ///
        /// The `secondary_disk` block includes:
        #[builder(into)]
        pub secondary_disk: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::DiskAsyncReplicationSecondaryDisk,
        >,
    }
    #[allow(dead_code)]
    pub struct DiskAsyncReplicationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The primary disk (source of replication).
        pub primary_disk: pulumi_gestalt_rust::Output<String>,
        /// The secondary disk (target of replication). You can specify only one value. Structure is documented below.
        ///
        /// The `secondary_disk` block includes:
        pub secondary_disk: pulumi_gestalt_rust::Output<
            super::super::types::compute::DiskAsyncReplicationSecondaryDisk,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DiskAsyncReplicationArgs,
    ) -> DiskAsyncReplicationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DiskAsyncReplicationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DiskAsyncReplicationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DiskAsyncReplicationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DiskAsyncReplicationResult {
        let primary_disk_binding = args.primary_disk.get_output(ctx);
        let secondary_disk_binding = args.secondary_disk.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/diskAsyncReplication:DiskAsyncReplication".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "primaryDisk".into(),
                    value: &primary_disk_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secondaryDisk".into(),
                    value: &secondary_disk_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DiskAsyncReplicationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            primary_disk: o.get_field("primaryDisk"),
            secondary_disk: o.get_field("secondaryDisk"),
        }
    }
}
