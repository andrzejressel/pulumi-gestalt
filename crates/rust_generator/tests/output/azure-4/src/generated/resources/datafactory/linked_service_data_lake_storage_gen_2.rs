/// Manages a Linked Service (connection) between Data Lake Storage Gen2 and Azure Data Factory.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleLinkedServiceDataLakeStorageGen2:
///     type: azure:datafactory:LinkedServiceDataLakeStorageGen2
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       servicePrincipalId: ${current.clientId}
///       servicePrincipalKey: exampleKey
///       tenant: 11111111-1111-1111-1111-111111111111
///       url: https://datalakestoragegen2
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Data Factory Data Lake Storage Gen2 Linked Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceDataLakeStorageGen2:LinkedServiceDataLakeStorageGen2 example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_service_data_lake_storage_gen_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceDataLakeStorageGen2Args {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Data Lake Storage Gen2 Linked Service:
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Linked Service.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The service principal id with which to authenticate against the Azure Data Lake Storage Gen2 account. Incompatible with `storage_account_key` and `use_managed_identity`.
        #[builder(into, default)]
        pub service_principal_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The service principal key with which to authenticate against the Azure Data Lake Storage Gen2 account.
        #[builder(into, default)]
        pub service_principal_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Storage Account Key with which to authenticate against the Azure Data Lake Storage Gen2 account. Incompatible with `service_principal_id`, `service_principal_key`, `tenant` and `use_managed_identity`.
        #[builder(into, default)]
        pub storage_account_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The tenant id or name in which the service principal exists to authenticate against the Azure Data Lake Storage Gen2 account.
        ///
        /// > **NOTE** If `service_principal_id` is used, `service_principal_key` and `tenant` are also required.
        #[builder(into, default)]
        pub tenant: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The endpoint for the Azure Data Lake Storage Gen2 service.
        ///
        /// > **NOTE** Users should specify only one of the following three authentication strategies: storage account key, managed identity, service principal.
        #[builder(into)]
        pub url: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to use the Data Factory's managed identity to authenticate against the Azure Data Lake Storage Gen2 account. Incompatible with `service_principal_id`, `service_principal_key`, `tenant` and `storage_account_key`.
        #[builder(into, default)]
        pub use_managed_identity: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceDataLakeStorageGen2Result {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Data Lake Storage Gen2 Linked Service:
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Linked Service.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        pub integration_runtime_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The service principal id with which to authenticate against the Azure Data Lake Storage Gen2 account. Incompatible with `storage_account_key` and `use_managed_identity`.
        pub service_principal_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The service principal key with which to authenticate against the Azure Data Lake Storage Gen2 account.
        pub service_principal_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Storage Account Key with which to authenticate against the Azure Data Lake Storage Gen2 account. Incompatible with `service_principal_id`, `service_principal_key`, `tenant` and `use_managed_identity`.
        pub storage_account_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The tenant id or name in which the service principal exists to authenticate against the Azure Data Lake Storage Gen2 account.
        ///
        /// > **NOTE** If `service_principal_id` is used, `service_principal_key` and `tenant` are also required.
        pub tenant: pulumi_gestalt_rust::Output<Option<String>>,
        /// The endpoint for the Azure Data Lake Storage Gen2 service.
        ///
        /// > **NOTE** Users should specify only one of the following three authentication strategies: storage account key, managed identity, service principal.
        pub url: pulumi_gestalt_rust::Output<String>,
        /// Whether to use the Data Factory's managed identity to authenticate against the Azure Data Lake Storage Gen2 account. Incompatible with `service_principal_id`, `service_principal_key`, `tenant` and `storage_account_key`.
        pub use_managed_identity: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinkedServiceDataLakeStorageGen2Args,
    ) -> LinkedServiceDataLakeStorageGen2Result {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_properties_binding = args
            .additional_properties
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let integration_runtime_name_binding = args
            .integration_runtime_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let service_principal_id_binding = args.service_principal_id.get_output(context);
        let service_principal_key_binding = args
            .service_principal_key
            .get_output(context);
        let storage_account_key_binding = args.storage_account_key.get_output(context);
        let tenant_binding = args.tenant.get_output(context);
        let url_binding = args.url.get_output(context);
        let use_managed_identity_binding = args.use_managed_identity.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceDataLakeStorageGen2:LinkedServiceDataLakeStorageGen2"
                .into(),
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
                    name: "integrationRuntimeName".into(),
                    value: &integration_runtime_name_binding.drop_type(),
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
                    name: "servicePrincipalId".into(),
                    value: &service_principal_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servicePrincipalKey".into(),
                    value: &service_principal_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountKey".into(),
                    value: &storage_account_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenant".into(),
                    value: &tenant_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "url".into(),
                    value: &url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useManagedIdentity".into(),
                    value: &use_managed_identity_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinkedServiceDataLakeStorageGen2Result {
            additional_properties: o.get_field("additionalProperties"),
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            integration_runtime_name: o.get_field("integrationRuntimeName"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            service_principal_id: o.get_field("servicePrincipalId"),
            service_principal_key: o.get_field("servicePrincipalKey"),
            storage_account_key: o.get_field("storageAccountKey"),
            tenant: o.get_field("tenant"),
            url: o.get_field("url"),
            use_managed_identity: o.get_field("useManagedIdentity"),
        }
    }
}
