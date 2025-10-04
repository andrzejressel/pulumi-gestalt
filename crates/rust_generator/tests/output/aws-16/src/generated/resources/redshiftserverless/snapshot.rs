/// Creates a new Amazon Redshift Serverless Snapshot.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = snapshot::create(
///         "example",
///         SnapshotArgs::builder()
///             .namespace_name("${exampleAwsRedshiftserverlessWorkgroup.namespaceName}")
///             .snapshot_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Serverless Snapshots using the `snapshot_name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshiftserverless/snapshot:Snapshot example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// The namespace to create a snapshot for.
        #[builder(into)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// How long to retain the created snapshot. Default value is `-1`.
        #[builder(into, default)]
        pub retention_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the snapshot.
        #[builder(into)]
        pub snapshot_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// All of the Amazon Web Services accounts that have access to restore a snapshot to a provisioned cluster.
        pub accounts_with_provisioned_restore_accesses: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// All of the Amazon Web Services accounts that have access to restore a snapshot to a namespace.
        pub accounts_with_restore_accesses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The username of the database within a snapshot.
        pub admin_username: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the snapshot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the KMS key used to encrypt the snapshot.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the namespace the snapshot was created from.
        pub namespace_arn: pulumi_gestalt_rust::Output<String>,
        /// The namespace to create a snapshot for.
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The owner Amazon Web Services; account of the snapshot.
        pub owner_account: pulumi_gestalt_rust::Output<String>,
        /// How long to retain the created snapshot. Default value is `-1`.
        pub retention_period: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the snapshot.
        pub snapshot_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotArgs,
    ) -> SnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let namespace_name_binding = args.namespace_name.get_output(context);
        let retention_period_binding = args.retention_period.get_output(context);
        let snapshot_name_binding = args.snapshot_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshiftserverless/snapshot:Snapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionPeriod".into(),
                    value: &retention_period_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotName".into(),
                    value: &snapshot_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SnapshotResult {
            id: o.get_field("id"),
            accounts_with_provisioned_restore_accesses: o
                .get_field("accountsWithProvisionedRestoreAccesses"),
            accounts_with_restore_accesses: o.get_field("accountsWithRestoreAccesses"),
            admin_username: o.get_field("adminUsername"),
            arn: o.get_field("arn"),
            kms_key_id: o.get_field("kmsKeyId"),
            namespace_arn: o.get_field("namespaceArn"),
            namespace_name: o.get_field("namespaceName"),
            owner_account: o.get_field("ownerAccount"),
            retention_period: o.get_field("retentionPeriod"),
            snapshot_name: o.get_field("snapshotName"),
        }
    }
}
