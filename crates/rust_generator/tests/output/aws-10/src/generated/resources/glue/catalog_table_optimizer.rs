/// Resource for managing an AWS Glue Catalog Table Optimizer.
///
/// ## Example Usage
///
/// ### Compaction Optimizer
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_table_optimizer::create(
///         "example",
///         CatalogTableOptimizerArgs::builder()
///             .catalog_id("123456789012")
///             .configuration(
///                 CatalogTableOptimizerConfiguration::builder()
///                     .enabled(true)
///                     .roleArn("arn:aws:iam::123456789012:role/example-role")
///                     .build_struct(),
///             )
///             .database_name("example_database")
///             .table_name("example_table")
///             .type_("compaction")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Snapshot Retention Optimizer
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_table_optimizer::create(
///         "example",
///         CatalogTableOptimizerArgs::builder()
///             .catalog_id("123456789012")
///             .configuration(
///                 CatalogTableOptimizerConfiguration::builder()
///                     .enabled(true)
///                     .retentionConfiguration(
///                         CatalogTableOptimizerConfigurationRetentionConfiguration::builder()
///                             .icebergConfiguration(
///                                 CatalogTableOptimizerConfigurationRetentionConfigurationIcebergConfiguration::builder()
///                                     .cleanExpiredFiles(true)
///                                     .numberOfSnapshotsToRetain(3)
///                                     .snapshotRetentionPeriodInDays(7)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .roleArn("arn:aws:iam::123456789012:role/example-role")
///                     .build_struct(),
///             )
///             .database_name("example_database")
///             .table_name("example_table")
///             .type_("retention")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Orphan File Deletion Optimizer
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_table_optimizer::create(
///         "example",
///         CatalogTableOptimizerArgs::builder()
///             .catalog_id("123456789012")
///             .configuration(
///                 CatalogTableOptimizerConfiguration::builder()
///                     .enabled(true)
///                     .orphanFileDeletionConfiguration(
///                         CatalogTableOptimizerConfigurationOrphanFileDeletionConfiguration::builder()
///                             .icebergConfiguration(
///                                 CatalogTableOptimizerConfigurationOrphanFileDeletionConfigurationIcebergConfiguration::builder()
///                                     .location("s3://example-bucket/example_table/")
///                                     .orphanFileRetentionPeriodInDays(7)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .roleArn("arn:aws:iam::123456789012:role/example-role")
///                     .build_struct(),
///             )
///             .database_name("example_database")
///             .table_name("example_table")
///             .type_("orphan_file_deletion")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Catalog Table Optimizer using the `catalog_id,database_name,table_name,type`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/catalogTableOptimizer:CatalogTableOptimizer example 123456789012,example_database,example_table,compaction
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod catalog_table_optimizer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CatalogTableOptimizerArgs {
        /// The Catalog ID of the table.
        #[builder(into)]
        pub catalog_id: pulumi_gestalt_rust::Input<String>,
        /// A configuration block that defines the table optimizer settings. See Configuration for additional details.
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::Input<
            Option<super::super::types::glue::CatalogTableOptimizerConfiguration>,
        >,
        /// The name of the database in the catalog in which the table resides.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::Input<String>,
        /// The name of the table.
        #[builder(into)]
        pub table_name: pulumi_gestalt_rust::Input<String>,
        /// The type of table optimizer. Valid values are `compaction`, `retention`, and `orphan_file_deletion`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct CatalogTableOptimizerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The Catalog ID of the table.
        pub catalog_id: pulumi_gestalt_rust::Output<String>,
        /// A configuration block that defines the table optimizer settings. See Configuration for additional details.
        pub configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::CatalogTableOptimizerConfiguration>,
        >,
        /// The name of the database in the catalog in which the table resides.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the table.
        pub table_name: pulumi_gestalt_rust::Output<String>,
        /// The type of table optimizer. Valid values are `compaction`, `retention`, and `orphan_file_deletion`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CatalogTableOptimizerArgs,
    ) -> CatalogTableOptimizerResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CatalogTableOptimizerArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> CatalogTableOptimizerResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CatalogTableOptimizerArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> CatalogTableOptimizerResult {
        let catalog_id_binding = args.catalog_id.get_output(ctx);
        let configuration_binding = args.configuration.get_output(ctx);
        let database_name_binding = args.database_name.get_output(ctx);
        let table_name_binding = args.table_name.get_output(ctx);
        let type__binding = args.type_.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/catalogTableOptimizer:CatalogTableOptimizer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableName".into(),
                    value: &table_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        CatalogTableOptimizerResult {
            id: o.get_id(),
            urn: o.get_urn(),
            catalog_id: o.get_field("catalogId"),
            configuration: o.get_field("configuration"),
            database_name: o.get_field("databaseName"),
            table_name: o.get_field("tableName"),
            type_: o.get_field("type"),
        }
    }
}
