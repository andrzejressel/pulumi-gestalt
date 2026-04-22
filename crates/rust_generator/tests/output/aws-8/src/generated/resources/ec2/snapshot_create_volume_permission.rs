/// Adds permission to create volumes off of a given EBS Snapshot.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = volume::create(
///         "example",
///         VolumeArgs::builder().availability_zone("us-west-2a").size(40).build_struct(),
///     );
///     let examplePerm = snapshot_create_volume_permission::create(
///         "examplePerm",
///         SnapshotCreateVolumePermissionArgs::builder()
///             .account_id("12345678")
///             .snapshot_id("${exampleSnapshot.id}")
///             .build_struct(),
///     );
///     let exampleSnapshot = snapshot::create(
///         "exampleSnapshot",
///         SnapshotArgs::builder().volume_id("${example.id}").build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod snapshot_create_volume_permission {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotCreateVolumePermissionArgs {
        /// An AWS Account ID to add create volume permissions. The AWS Account cannot be the snapshot's owner
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::Input<String>,
        /// A snapshot ID
        #[builder(into)]
        pub snapshot_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotCreateVolumePermissionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// An AWS Account ID to add create volume permissions. The AWS Account cannot be the snapshot's owner
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// A snapshot ID
        pub snapshot_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotCreateVolumePermissionArgs,
    ) -> SnapshotCreateVolumePermissionResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotCreateVolumePermissionArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SnapshotCreateVolumePermissionResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotCreateVolumePermissionArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SnapshotCreateVolumePermissionResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let snapshot_id_binding = args.snapshot_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/snapshotCreateVolumePermission:SnapshotCreateVolumePermission"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotId".into(),
                    value: &snapshot_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SnapshotCreateVolumePermissionResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            snapshot_id: o.get_field("snapshotId"),
        }
    }
}
