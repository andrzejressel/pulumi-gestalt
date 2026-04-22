/// Resource for managing an EBS (Elastic Block Storage) Fast Snapshot Restore.
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
///     let example = fast_snapshot_restore::create(
///         "example",
///         FastSnapshotRestoreArgs::builder()
///             .availability_zone("us-west-2a")
///             .snapshot_id("${exampleAwsEbsSnapshot.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EC2 (Elastic Compute Cloud) EBS Fast Snapshot Restore using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ebs/fastSnapshotRestore:FastSnapshotRestore example us-west-2a,snap-abcdef123456
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod fast_snapshot_restore {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FastSnapshotRestoreArgs {
        /// Availability zone in which to enable fast snapshot restores.
        #[builder(into)]
        pub availability_zone: pulumi_gestalt_rust::Input<String>,
        /// ID of the snapshot.
        #[builder(into)]
        pub snapshot_id: pulumi_gestalt_rust::Input<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::Input<
            Option<super::super::types::ebs::FastSnapshotRestoreTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct FastSnapshotRestoreResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Availability zone in which to enable fast snapshot restores.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// ID of the snapshot.
        pub snapshot_id: pulumi_gestalt_rust::Output<String>,
        /// State of fast snapshot restores. Valid values are `enabling`, `optimizing`, `enabled`, `disabling`, `disabled`.
        pub state: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::ebs::FastSnapshotRestoreTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FastSnapshotRestoreArgs,
    ) -> FastSnapshotRestoreResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FastSnapshotRestoreArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> FastSnapshotRestoreResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FastSnapshotRestoreArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> FastSnapshotRestoreResult {
        let availability_zone_binding = args.availability_zone.get_output(ctx);
        let snapshot_id_binding = args.snapshot_id.get_output(ctx);
        let timeouts_binding = args.timeouts.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ebs/fastSnapshotRestore:FastSnapshotRestore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotId".into(),
                    value: &snapshot_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        FastSnapshotRestoreResult {
            id: o.get_id(),
            urn: o.get_urn(),
            availability_zone: o.get_field("availabilityZone"),
            snapshot_id: o.get_field("snapshotId"),
            state: o.get_field("state"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
