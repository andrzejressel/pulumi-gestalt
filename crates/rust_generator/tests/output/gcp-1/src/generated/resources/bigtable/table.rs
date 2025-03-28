/// Creates a Google Cloud Bigtable table inside an instance. For more information see
/// [the official documentation](https://cloud.google.com/bigtable/) and
/// [API](https://cloud.google.com/bigtable/docs/go/reference).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .clusters(
///                 vec![
///                     InstanceCluster::builder().clusterId("tf-instance-cluster")
///                     .numNodes(3).storageType("HDD").zone("us-central1-b").build_struct(),
///                 ],
///             )
///             .name("tf-instance")
///             .build_struct(),
///     );
///     let table = table::create(
///         "table",
///         TableArgs::builder()
///             .automated_backup_policy(
///                 TableAutomatedBackupPolicy::builder()
///                     .frequency("24h0m0s")
///                     .retentionPeriod("72h0m0s")
///                     .build_struct(),
///             )
///             .change_stream_retention("24h0m0s")
///             .column_families(
///                 vec![
///                     TableColumnFamily::builder().family("family-first").build_struct(),
///                     TableColumnFamily::builder().family("family-second"). type ("intsum")
///                     .build_struct(), TableColumnFamily::builder().family("family-third").
///                     type
///                     ("        {\n\t\t\t\t\t\"aggregateType\": {\n\t\t\t\t\t\t\"max\": {},\n\t\t\t\t\t\t\"inputType\": {\n\t\t\t\t\t\t\t\"int64Type\": {\n\t\t\t\t\t\t\t\t\"encoding\": {\n\t\t\t\t\t\t\t\t\t\"bigEndianBytes\": {}\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t}\n")
///                     .build_struct(),
///                 ],
///             )
///             .instance_name("${instance.name}")
///             .name("tf-table")
///             .split_keys(vec!["a", "b", "c",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// -> **Fields affected by import** The following fields can't be read and will show diffs if set in config when imported: `split_keys`
///
/// Bigtable Tables can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/instances/{{instance_name}}/tables/{{name}}`
///
/// * `{{project}}/{{instance_name}}/{{name}}`
///
/// * `{{instance_name}}/{{name}}`
///
/// When using the `pulumi import` command, Bigtable Tables can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigtable/table:Table default projects/{{project}}/instances/{{instance_name}}/tables/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigtable/table:Table default {{project}}/{{instance_name}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigtable/table:Table default {{instance_name}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableArgs {
        /// Defines an automated backup policy for a table, specified by Retention Period and Frequency. To disable, set both Retention Period and Frequency to 0.
        ///
        /// -----
        #[builder(into, default)]
        pub automated_backup_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigtable::TableAutomatedBackupPolicy>,
        >,
        /// Duration to retain change stream data for the table. Set to 0 to disable. Must be between 1 and 7 days.
        #[builder(into, default)]
        pub change_stream_retention: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A group of columns within a table which share a common configuration. This can be specified multiple times. Structure is documented below.
        #[builder(into, default)]
        pub column_families: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::bigtable::TableColumnFamily>>,
        >,
        /// A field to make the table protected against data loss i.e. when set to PROTECTED, deleting the table, the column families in the table, and the instance containing the table would be prohibited. If not provided, deletion protection will be set to UNPROTECTED.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Bigtable instance.
        #[builder(into)]
        pub instance_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the table. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of predefined keys to split the table on.
        /// !> **Warning:** Modifying the `split_keys` of an existing table will cause the provider
        /// to delete/recreate the entire `gcp.bigtable.Table` resource.
        #[builder(into, default)]
        pub split_keys: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct TableResult {
        /// Defines an automated backup policy for a table, specified by Retention Period and Frequency. To disable, set both Retention Period and Frequency to 0.
        ///
        /// -----
        pub automated_backup_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigtable::TableAutomatedBackupPolicy>,
        >,
        /// Duration to retain change stream data for the table. Set to 0 to disable. Must be between 1 and 7 days.
        pub change_stream_retention: pulumi_gestalt_rust::Output<String>,
        /// A group of columns within a table which share a common configuration. This can be specified multiple times. Structure is documented below.
        pub column_families: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::bigtable::TableColumnFamily>>,
        >,
        /// A field to make the table protected against data loss i.e. when set to PROTECTED, deleting the table, the column families in the table, and the instance containing the table would be prohibited. If not provided, deletion protection will be set to UNPROTECTED.
        pub deletion_protection: pulumi_gestalt_rust::Output<String>,
        /// The name of the Bigtable instance.
        pub instance_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the table. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A list of predefined keys to split the table on.
        /// !> **Warning:** Modifying the `split_keys` of an existing table will cause the provider
        /// to delete/recreate the entire `gcp.bigtable.Table` resource.
        pub split_keys: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableArgs,
    ) -> TableResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automated_backup_policy_binding = args
            .automated_backup_policy
            .get_output(context);
        let change_stream_retention_binding = args
            .change_stream_retention
            .get_output(context);
        let column_families_binding = args.column_families.get_output(context);
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let instance_name_binding = args.instance_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let split_keys_binding = args.split_keys.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigtable/table:Table".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automatedBackupPolicy".into(),
                    value: &automated_backup_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "changeStreamRetention".into(),
                    value: &change_stream_retention_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "columnFamilies".into(),
                    value: &column_families_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceName".into(),
                    value: &instance_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "splitKeys".into(),
                    value: &split_keys_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TableResult {
            automated_backup_policy: o.get_field("automatedBackupPolicy"),
            change_stream_retention: o.get_field("changeStreamRetention"),
            column_families: o.get_field("columnFamilies"),
            deletion_protection: o.get_field("deletionProtection"),
            instance_name: o.get_field("instanceName"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            split_keys: o.get_field("splitKeys"),
        }
    }
}
