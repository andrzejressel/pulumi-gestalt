/// Creates a Redshift cluster snapshot
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:redshift:ClusterSnapshot
///     properties:
///       clusterSnapshotName: example
///       clusterSnapshotContent:
///         fn::toJSON:
///           AllowDBUserOverride: '1'
///           Client_ID: ExampleClientID
///           App_ID: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Cluster Snapshots using `snapshot_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/clusterSnapshot:ClusterSnapshot test example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod cluster_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterSnapshotArgs {
        /// The cluster identifier for which you want a snapshot.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::Input<String>,
        /// The number of days that a manual snapshot is retained. If the value is `-1`, the manual snapshot is retained indefinitely. Valid values are -1 and between `1` and `3653`.
        #[builder(into, default)]
        pub manual_snapshot_retention_period: pulumi_gestalt_rust::Input<Option<i32>>,
        /// A unique identifier for the snapshot that you are requesting. This identifier must be unique for all snapshots within the Amazon Web Services account.
        #[builder(into)]
        pub snapshot_identifier: pulumi_gestalt_rust::Input<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterSnapshotResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the snapshot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The cluster identifier for which you want a snapshot.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The Key Management Service (KMS) key ID of the encryption key that was used to encrypt data in the cluster from which the snapshot was taken.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The number of days that a manual snapshot is retained. If the value is `-1`, the manual snapshot is retained indefinitely. Valid values are -1 and between `1` and `3653`.
        pub manual_snapshot_retention_period: pulumi_gestalt_rust::Output<Option<i32>>,
        /// For manual snapshots, the Amazon Web Services account used to create or copy the snapshot. For automatic snapshots, the owner of the cluster. The owner can perform all snapshot actions, such as sharing a manual snapshot.
        pub owner_account: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for the snapshot that you are requesting. This identifier must be unique for all snapshots within the Amazon Web Services account.
        pub snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterSnapshotArgs,
    ) -> ClusterSnapshotResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterSnapshotArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ClusterSnapshotResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterSnapshotArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ClusterSnapshotResult {
        let cluster_identifier_binding = args.cluster_identifier.get_output(ctx);
        let manual_snapshot_retention_period_binding = args
            .manual_snapshot_retention_period
            .get_output(ctx);
        let snapshot_identifier_binding = args.snapshot_identifier.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/clusterSnapshot:ClusterSnapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "manualSnapshotRetentionPeriod".into(),
                    value: &manual_snapshot_retention_period_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotIdentifier".into(),
                    value: &snapshot_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ClusterSnapshotResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            kms_key_id: o.get_field("kmsKeyId"),
            manual_snapshot_retention_period: o
                .get_field("manualSnapshotRetentionPeriod"),
            owner_account: o.get_field("ownerAccount"),
            snapshot_identifier: o.get_field("snapshotIdentifier"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
