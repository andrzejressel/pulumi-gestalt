/// Manages an Azure Parquet Dataset inside an Azure Data Factory.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleDatasetParquet = dataset_parquet::create(
///         "exampleDatasetParquet",
///         DatasetParquetArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .http_server_location(
///                 DatasetParquetHttpServerLocation::builder()
///                     .filename("fizz.txt")
///                     .path("foo/bar/")
///                     .relativeUrl("http://www.bing.com")
///                     .build_struct(),
///             )
///             .linked_service_name("${exampleLinkedServiceWeb.name}")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLinkedServiceWeb = linked_service_web::create(
///         "exampleLinkedServiceWeb",
///         LinkedServiceWebArgs::builder()
///             .authentication_type("Anonymous")
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .url("https://www.bing.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Datasets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/datasetParquet:DatasetParquet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/datasets/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dataset_parquet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetParquetArgs {
        /// A map of additional properties to associate with the Data Factory Dataset.
        ///
        /// The following supported locations for a Parquet Dataset:
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A `azure_blob_fs_location` block as defined below.
        #[builder(into, default)]
        pub azure_blob_fs_location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::DatasetParquetAzureBlobFsLocation>,
        >,
        /// A `azure_blob_storage_location` block as defined below.
        ///
        /// The following supported arguments are specific to Parquet Dataset:
        #[builder(into, default)]
        pub azure_blob_storage_location: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::DatasetParquetAzureBlobStorageLocation,
            >,
        >,
        /// The compression codec used to read/write text files. Valid values are `bzip2`, `gzip`, `deflate`, `ZipDeflate`, `TarGzip`, `Tar`, `snappy`, or `lz4`. Please note these values are case-sensitive.
        #[builder(into, default)]
        pub compression_codec: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the compression level. Possible values are `Optimal` and `Fastest`,
        #[builder(into, default)]
        pub compression_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Data Factory ID in which to associate the Dataset with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Dataset.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `http_server_location` block as defined below.
        #[builder(into, default)]
        pub http_server_location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::DatasetParquetHttpServerLocation>,
        >,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        #[builder(into)]
        pub linked_service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Dataset.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `schema_column` block as defined below.
        #[builder(into, default)]
        pub schema_columns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datafactory::DatasetParquetSchemaColumn>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DatasetParquetResult {
        /// A map of additional properties to associate with the Data Factory Dataset.
        ///
        /// The following supported locations for a Parquet Dataset:
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A `azure_blob_fs_location` block as defined below.
        pub azure_blob_fs_location: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::DatasetParquetAzureBlobFsLocation>,
        >,
        /// A `azure_blob_storage_location` block as defined below.
        ///
        /// The following supported arguments are specific to Parquet Dataset:
        pub azure_blob_storage_location: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::DatasetParquetAzureBlobStorageLocation,
            >,
        >,
        /// The compression codec used to read/write text files. Valid values are `bzip2`, `gzip`, `deflate`, `ZipDeflate`, `TarGzip`, `Tar`, `snappy`, or `lz4`. Please note these values are case-sensitive.
        pub compression_codec: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the compression level. Possible values are `Optimal` and `Fastest`,
        pub compression_level: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Data Factory ID in which to associate the Dataset with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Dataset.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        pub folder: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `http_server_location` block as defined below.
        pub http_server_location: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::DatasetParquetHttpServerLocation>,
        >,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        pub linked_service_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Dataset.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `schema_column` block as defined below.
        pub schema_columns: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datafactory::DatasetParquetSchemaColumn>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatasetParquetArgs,
    ) -> DatasetParquetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_properties_binding = args
            .additional_properties
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let azure_blob_fs_location_binding = args
            .azure_blob_fs_location
            .get_output(context);
        let azure_blob_storage_location_binding = args
            .azure_blob_storage_location
            .get_output(context);
        let compression_codec_binding = args.compression_codec.get_output(context);
        let compression_level_binding = args.compression_level.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let folder_binding = args.folder.get_output(context);
        let http_server_location_binding = args.http_server_location.get_output(context);
        let linked_service_name_binding = args.linked_service_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let schema_columns_binding = args.schema_columns.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/datasetParquet:DatasetParquet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalProperties".into(),
                    value: &additional_properties_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureBlobFsLocation".into(),
                    value: &azure_blob_fs_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureBlobStorageLocation".into(),
                    value: &azure_blob_storage_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "compressionCodec".into(),
                    value: &compression_codec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "compressionLevel".into(),
                    value: &compression_level_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: &folder_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpServerLocation".into(),
                    value: &http_server_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkedServiceName".into(),
                    value: &linked_service_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schemaColumns".into(),
                    value: &schema_columns_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatasetParquetResult {
            additional_properties: o.get_field("additionalProperties"),
            annotations: o.get_field("annotations"),
            azure_blob_fs_location: o.get_field("azureBlobFsLocation"),
            azure_blob_storage_location: o.get_field("azureBlobStorageLocation"),
            compression_codec: o.get_field("compressionCodec"),
            compression_level: o.get_field("compressionLevel"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            folder: o.get_field("folder"),
            http_server_location: o.get_field("httpServerLocation"),
            linked_service_name: o.get_field("linkedServiceName"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            schema_columns: o.get_field("schemaColumns"),
        }
    }
}
