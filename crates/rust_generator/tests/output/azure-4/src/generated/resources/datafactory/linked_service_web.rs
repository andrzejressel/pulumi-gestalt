/// Manages a Linked Service (connection) between a Web Server and Azure Data Factory.
///
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
///             .url("http://www.bing.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceWeb:LinkedServiceWeb example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_service_web {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceWebArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Web Linked Service:
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The type of authentication used to connect to the web table source. Valid options are `Anonymous`, `Basic` and `ClientCertificate`.
        #[builder(into)]
        pub authentication_type: pulumi_gestalt_rust::InputOrOutput<String>,
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
        /// The password for Basic authentication. Required if `authentication_type` sets to `Basic`.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of the web service endpoint (e.g. <https://www.microsoft.com>).
        #[builder(into)]
        pub url: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The username for Basic authentication. Required if `authentication_type` sets to `Basic`.
        #[builder(into, default)]
        pub username: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceWebResult {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to Web Linked Service:
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The type of authentication used to connect to the web table source. Valid options are `Anonymous`, `Basic` and `ClientCertificate`.
        pub authentication_type: pulumi_gestalt_rust::Output<String>,
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
        /// The password for Basic authentication. Required if `authentication_type` sets to `Basic`.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URL of the web service endpoint (e.g. <https://www.microsoft.com>).
        pub url: pulumi_gestalt_rust::Output<String>,
        /// The username for Basic authentication. Required if `authentication_type` sets to `Basic`.
        pub username: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinkedServiceWebArgs,
    ) -> LinkedServiceWebResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_properties_binding = args
            .additional_properties
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let authentication_type_binding = args.authentication_type.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let integration_runtime_name_binding = args
            .integration_runtime_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let password_binding = args.password.get_output(context);
        let url_binding = args.url.get_output(context);
        let username_binding = args.username.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceWeb:LinkedServiceWeb".into(),
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
                    name: "authenticationType".into(),
                    value: &authentication_type_binding.drop_type(),
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
                    name: "password".into(),
                    value: &password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "url".into(),
                    value: &url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "username".into(),
                    value: &username_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinkedServiceWebResult {
            additional_properties: o.get_field("additionalProperties"),
            annotations: o.get_field("annotations"),
            authentication_type: o.get_field("authenticationType"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            integration_runtime_name: o.get_field("integrationRuntimeName"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            password: o.get_field("password"),
            url: o.get_field("url"),
            username: o.get_field("username"),
        }
    }
}
