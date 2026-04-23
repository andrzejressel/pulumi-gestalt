/// Schema defines the structure and layout of a type of document data.
///
///
/// To get more information about Schema, see:
///
/// * [API documentation](https://cloud.google.com/generative-ai-app-builder/docs/reference/rest/v1/projects.locations.collections.dataStores.schemas)
/// * How-to Guides
///     * [Provide a schema for your data store](https://cloud.google.com/generative-ai-app-builder/docs/provide-schema)
///
/// ## Example Usage
///
/// ### Discoveryengine Schema Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = schema::create(
///         "basic",
///         SchemaArgs::builder()
///             .data_store_id("${basicDataStore.dataStoreId}")
///             .json_schema(
///                 "{\"$schema\":\"https://json-schema.org/draft/2020-12/schema\",\"datetime_detection\":true,\"type\":\"object\",\"geolocation_detection\":true}",
///             )
///             .location("${basicDataStore.location}")
///             .schema_id("schema-id")
///             .build_struct(),
///     );
///     let basicDataStore = data_store::create(
///         "basicDataStore",
///         DataStoreArgs::builder()
///             .content_config("NO_CONTENT")
///             .create_advanced_site_search(false)
///             .data_store_id("data-store-id")
///             .display_name("tf-test-structured-datastore")
///             .industry_vertical("GENERIC")
///             .location("global")
///             .skip_default_schema_creation(true)
///             .solution_types(vec!["SOLUTION_TYPE_SEARCH",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Schema can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/collections/default_collection/dataStores/{{data_store_id}}/schemas/{{schema_id}}`
///
/// * `{{project}}/{{location}}/{{data_store_id}}/{{schema_id}}`
///
/// * `{{location}}/{{data_store_id}}/{{schema_id}}`
///
/// When using the `pulumi import` command, Schema can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/schema:Schema default projects/{{project}}/locations/{{location}}/collections/default_collection/dataStores/{{data_store_id}}/schemas/{{schema_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/schema:Schema default {{project}}/{{location}}/{{data_store_id}}/{{schema_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/schema:Schema default {{location}}/{{data_store_id}}/{{schema_id}}
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod schema {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchemaArgs {
        /// The unique id of the data store.
        #[builder(into)]
        pub data_store_id: pulumi_gestalt_rust::Input<String>,
        /// The JSON representation of the schema.
        #[builder(into, default)]
        pub json_schema: pulumi_gestalt_rust::Input<Option<String>>,
        /// The geographic location where the data store should reside. The value can
        /// only be one of "global", "us" and "eu".
        #[builder(into)]
        pub location: pulumi_gestalt_rust::Input<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::Input<Option<String>>,
        /// The unique id of the schema.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub schema_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct SchemaResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The unique id of the data store.
        pub data_store_id: pulumi_gestalt_rust::Output<String>,
        /// The JSON representation of the schema.
        pub json_schema: pulumi_gestalt_rust::Output<Option<String>>,
        /// The geographic location where the data store should reside. The value can
        /// only be one of "global", "us" and "eu".
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The unique full resource name of the schema. Values are of the format
        /// `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/schemas/{schema_id}`.
        /// This field must be a UTF-8 encoded string with a length limit of 1024
        /// characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The unique id of the schema.
        ///
        ///
        /// - - -
        pub schema_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SchemaArgs,
    ) -> SchemaResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SchemaArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> SchemaResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SchemaArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> SchemaResult {
        let data_store_id_binding = args.data_store_id.get_output(ctx);
        let json_schema_binding = args.json_schema.get_output(ctx);
        let location_binding = args.location.get_output(ctx);
        let project_binding = args.project.get_output(ctx);
        let schema_id_binding = args.schema_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:discoveryengine/schema:Schema".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataStoreId".into(),
                    value: &data_store_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jsonSchema".into(),
                    value: &json_schema_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schemaId".into(),
                    value: &schema_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        SchemaResult {
            id: o.get_id(),
            urn: o.get_urn(),
            data_store_id: o.get_field("dataStoreId"),
            json_schema: o.get_field("jsonSchema"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            schema_id: o.get_field("schemaId"),
        }
    }
}
