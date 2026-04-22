/// Resource for managing an AWS Lake Formation Data Cells Filter.
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
///     let example = data_cells_filter::create(
///         "example",
///         DataCellsFilterArgs::builder()
///             .table_data(
///                 DataCellsFilterTableData::builder()
///                     .columnNames(vec!["my_column",])
///                     .databaseName("${test.name}")
///                     .name("example")
///                     .rowFilter(
///                         DataCellsFilterTableDataRowFilter::builder()
///                             .filterExpression("my_column='example'")
///                             .build_struct(),
///                     )
///                     .tableCatalogId("${current.accountId}")
///                     .tableName("${testAwsGlueCatalogTable.name}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lake Formation Data Cells Filter using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:lakeformation/dataCellsFilter:DataCellsFilter example database_name,name,table_catalog_id,table_name
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod data_cells_filter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataCellsFilterArgs {
        /// Information about the data cells filter. See Table Data below for details.
        #[builder(into, default)]
        pub table_data: pulumi_gestalt_rust::Input<
            Option<super::super::types::lakeformation::DataCellsFilterTableData>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::Input<
            Option<super::super::types::lakeformation::DataCellsFilterTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataCellsFilterResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Information about the data cells filter. See Table Data below for details.
        pub table_data: pulumi_gestalt_rust::Output<
            Option<super::super::types::lakeformation::DataCellsFilterTableData>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::lakeformation::DataCellsFilterTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataCellsFilterArgs,
    ) -> DataCellsFilterResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataCellsFilterArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DataCellsFilterResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataCellsFilterArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DataCellsFilterResult {
        let table_data_binding = args.table_data.get_output(ctx);
        let timeouts_binding = args.timeouts.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lakeformation/dataCellsFilter:DataCellsFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableData".into(),
                    value: &table_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DataCellsFilterResult {
            id: o.get_id(),
            urn: o.get_urn(),
            table_data: o.get_field("tableData"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
