/// Manages an Azure Blob Dataset inside an Azure Data Factory.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleLinkedServiceAzureBlobStorage:
///     type: azure:datafactory:LinkedServiceAzureBlobStorage
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       connectionString: ${example.primaryConnectionString}
///   exampleDatasetAzureBlob:
///     type: azure:datafactory:DatasetAzureBlob
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       linkedServiceName: ${exampleLinkedServiceAzureBlobStorage.name}
///       path: foo
///       filename: bar.png
/// variables:
///   example:
///     fn::invoke:
///       function: azure:storage:getAccount
///       arguments:
///         name: storageaccountname
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Data Factory Datasets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/datasetAzureBlob:DatasetAzureBlob example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/datasets/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dataset_azure_blob {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetAzureBlobArgs {
        /// A map of additional properties to associate with the Data Factory Dataset.
        ///
        /// The following supported arguments are specific to Azure Blob Dataset:
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Dataset.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is the `filename` using dynamic expression, function or system variables? Defaults to `false`.
        #[builder(into, default)]
        pub dynamic_filename_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Is the `path` using dynamic expression, function or system variables? Defaults to `false`.
        #[builder(into, default)]
        pub dynamic_path_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The filename of the Azure Blob.
        #[builder(into, default)]
        pub filename: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        /// The path of the Azure Blob.
        #[builder(into, default)]
        pub path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `schema_column` block as defined below.
        #[builder(into, default)]
        pub schema_columns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datafactory::DatasetAzureBlobSchemaColumn>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DatasetAzureBlobResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A map of additional properties to associate with the Data Factory Dataset.
        ///
        /// The following supported arguments are specific to Azure Blob Dataset:
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Dataset.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Is the `filename` using dynamic expression, function or system variables? Defaults to `false`.
        pub dynamic_filename_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Is the `path` using dynamic expression, function or system variables? Defaults to `false`.
        pub dynamic_path_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The filename of the Azure Blob.
        pub filename: pulumi_gestalt_rust::Output<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        pub folder: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        pub linked_service_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Dataset.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The path of the Azure Blob.
        pub path: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `schema_column` block as defined below.
        pub schema_columns: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datafactory::DatasetAzureBlobSchemaColumn>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatasetAzureBlobArgs,
    ) -> DatasetAzureBlobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_properties_binding = args
            .additional_properties
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let dynamic_filename_enabled_binding = args
            .dynamic_filename_enabled
            .get_output(context);
        let dynamic_path_enabled_binding = args.dynamic_path_enabled.get_output(context);
        let filename_binding = args.filename.get_output(context);
        let folder_binding = args.folder.get_output(context);
        let linked_service_name_binding = args.linked_service_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let path_binding = args.path.get_output(context);
        let schema_columns_binding = args.schema_columns.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/datasetAzureBlob:DatasetAzureBlob".into(),
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
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dynamicFilenameEnabled".into(),
                    value: &dynamic_filename_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dynamicPathEnabled".into(),
                    value: &dynamic_path_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filename".into(),
                    value: &filename_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: &folder_binding.drop_type(),
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
                    name: "path".into(),
                    value: &path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schemaColumns".into(),
                    value: &schema_columns_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatasetAzureBlobResult {
            id: o.get_field("id"),
            additional_properties: o.get_field("additionalProperties"),
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            dynamic_filename_enabled: o.get_field("dynamicFilenameEnabled"),
            dynamic_path_enabled: o.get_field("dynamicPathEnabled"),
            filename: o.get_field("filename"),
            folder: o.get_field("folder"),
            linked_service_name: o.get_field("linkedServiceName"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            path: o.get_field("path"),
            schema_columns: o.get_field("schemaColumns"),
        }
    }
}
