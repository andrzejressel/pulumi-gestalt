/// Data store is a collection of websites and documents used to find answers for
/// end-user's questions in Discovery Engine (a.k.a. Vertex AI Search and
/// Conversation).
///
///
/// To get more information about DataStore, see:
///
/// * [API documentation](https://cloud.google.com/generative-ai-app-builder/docs/reference/rest/v1/projects.locations.collections.dataStores)
/// * How-to Guides
///     * [Create a search data store](https://cloud.google.com/generative-ai-app-builder/docs/create-data-store-es)
///
/// ## Example Usage
///
/// ### Discoveryengine Datastore Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = data_store::create(
///         "basic",
///         DataStoreArgs::builder()
///             .content_config("NO_CONTENT")
///             .create_advanced_site_search(false)
///             .data_store_id("data-store-id")
///             .display_name("tf-test-structured-datastore")
///             .industry_vertical("GENERIC")
///             .location("global")
///             .skip_default_schema_creation(false)
///             .solution_types(vec!["SOLUTION_TYPE_SEARCH",])
///             .build_struct(),
///     );
/// }
/// ```
/// ### Discoveryengine Datastore Document Processing Config
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let documentProcessingConfig = data_store::create(
///         "documentProcessingConfig",
///         DataStoreArgs::builder()
///             .content_config("NO_CONTENT")
///             .create_advanced_site_search(false)
///             .data_store_id("data-store-id")
///             .display_name("tf-test-structured-datastore")
///             .document_processing_config(
///                 DataStoreDocumentProcessingConfig::builder()
///                     .defaultParsingConfig(
///                         DataStoreDocumentProcessingConfigDefaultParsingConfig::builder()
///                             .digitalParsingConfig(
///                                 DataStoreDocumentProcessingConfigDefaultParsingConfigDigitalParsingConfig::builder()
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .parsingConfigOverrides(
///                         vec![
///                             DataStoreDocumentProcessingConfigParsingConfigOverride::builder()
///                             .fileType("pdf")
///                             .ocrParsingConfig(DataStoreDocumentProcessingConfigParsingConfigOverrideOcrParsingConfig::builder()
///                             .useNativeText(true).build_struct()).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .industry_vertical("GENERIC")
///             .location("global")
///             .solution_types(vec!["SOLUTION_TYPE_SEARCH",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// DataStore can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/collections/default_collection/dataStores/{{data_store_id}}`
///
/// * `{{project}}/{{location}}/{{data_store_id}}`
///
/// * `{{location}}/{{data_store_id}}`
///
/// When using the `pulumi import` command, DataStore can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/dataStore:DataStore default projects/{{project}}/locations/{{location}}/collections/default_collection/dataStores/{{data_store_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/dataStore:DataStore default {{project}}/{{location}}/{{data_store_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:discoveryengine/dataStore:DataStore default {{location}}/{{data_store_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataStoreArgs {
        /// The content config of the data store.
        /// Possible values are: `NO_CONTENT`, `CONTENT_REQUIRED`, `PUBLIC_WEBSITE`.
        #[builder(into)]
        pub content_config: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If true, an advanced data store for site search will be created. If the
        /// data store is not configured as site search (GENERIC vertical and
        /// PUBLIC_WEBSITE contentConfig), this flag will be ignored.
        #[builder(into, default)]
        pub create_advanced_site_search: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The unique id of the data store.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub data_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The display name of the data store. This field must be a UTF-8 encoded
        /// string with a length limit of 128 characters.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for Document understanding and enrichment.
        /// Structure is documented below.
        #[builder(into, default)]
        pub document_processing_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::discoveryengine::DataStoreDocumentProcessingConfig,
            >,
        >,
        /// The industry vertical that the data store registers.
        /// Possible values are: `GENERIC`, `MEDIA`, `HEALTHCARE_FHIR`.
        #[builder(into)]
        pub industry_vertical: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The geographic location where the data store should reside. The value can
        /// only be one of "global", "us" and "eu".
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A boolean flag indicating whether to skip the default schema creation for
        /// the data store. Only enable this flag if you are certain that the default
        /// schema is incompatible with your use case.
        /// If set to true, you must manually create a schema for the data store
        /// before any documents can be ingested.
        /// This flag cannot be specified if `data_store.starting_schema` is
        /// specified.
        #[builder(into, default)]
        pub skip_default_schema_creation: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The solutions that the data store enrolls.
        /// Each value may be one of: `SOLUTION_TYPE_RECOMMENDATION`, `SOLUTION_TYPE_SEARCH`, `SOLUTION_TYPE_CHAT`, `SOLUTION_TYPE_GENERATIVE_CHAT`.
        #[builder(into, default)]
        pub solution_types: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct DataStoreResult {
        /// The content config of the data store.
        /// Possible values are: `NO_CONTENT`, `CONTENT_REQUIRED`, `PUBLIC_WEBSITE`.
        pub content_config: pulumi_gestalt_rust::Output<String>,
        /// If true, an advanced data store for site search will be created. If the
        /// data store is not configured as site search (GENERIC vertical and
        /// PUBLIC_WEBSITE contentConfig), this flag will be ignored.
        pub create_advanced_site_search: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Timestamp when the DataStore was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The unique id of the data store.
        ///
        ///
        /// - - -
        pub data_store_id: pulumi_gestalt_rust::Output<String>,
        /// The id of the default Schema associated with this data store.
        pub default_schema_id: pulumi_gestalt_rust::Output<String>,
        /// The display name of the data store. This field must be a UTF-8 encoded
        /// string with a length limit of 128 characters.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration for Document understanding and enrichment.
        /// Structure is documented below.
        pub document_processing_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::discoveryengine::DataStoreDocumentProcessingConfig,
            >,
        >,
        /// The industry vertical that the data store registers.
        /// Possible values are: `GENERIC`, `MEDIA`, `HEALTHCARE_FHIR`.
        pub industry_vertical: pulumi_gestalt_rust::Output<String>,
        /// The geographic location where the data store should reside. The value can
        /// only be one of "global", "us" and "eu".
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The unique full resource name of the data store. Values are of the format
        /// `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}`.
        /// This field must be a UTF-8 encoded string with a length limit of 1024
        /// characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A boolean flag indicating whether to skip the default schema creation for
        /// the data store. Only enable this flag if you are certain that the default
        /// schema is incompatible with your use case.
        /// If set to true, you must manually create a schema for the data store
        /// before any documents can be ingested.
        /// This flag cannot be specified if `data_store.starting_schema` is
        /// specified.
        pub skip_default_schema_creation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The solutions that the data store enrolls.
        /// Each value may be one of: `SOLUTION_TYPE_RECOMMENDATION`, `SOLUTION_TYPE_SEARCH`, `SOLUTION_TYPE_CHAT`, `SOLUTION_TYPE_GENERATIVE_CHAT`.
        pub solution_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataStoreArgs,
    ) -> DataStoreResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let content_config_binding = args.content_config.get_output(context);
        let create_advanced_site_search_binding = args
            .create_advanced_site_search
            .get_output(context);
        let data_store_id_binding = args.data_store_id.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let document_processing_config_binding = args
            .document_processing_config
            .get_output(context);
        let industry_vertical_binding = args.industry_vertical.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let skip_default_schema_creation_binding = args
            .skip_default_schema_creation
            .get_output(context);
        let solution_types_binding = args.solution_types.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:discoveryengine/dataStore:DataStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentConfig".into(),
                    value: &content_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createAdvancedSiteSearch".into(),
                    value: &create_advanced_site_search_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataStoreId".into(),
                    value: &data_store_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "documentProcessingConfig".into(),
                    value: &document_processing_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "industryVertical".into(),
                    value: &industry_vertical_binding.drop_type(),
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
                    name: "skipDefaultSchemaCreation".into(),
                    value: &skip_default_schema_creation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "solutionTypes".into(),
                    value: &solution_types_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataStoreResult {
            content_config: o.get_field("contentConfig"),
            create_advanced_site_search: o.get_field("createAdvancedSiteSearch"),
            create_time: o.get_field("createTime"),
            data_store_id: o.get_field("dataStoreId"),
            default_schema_id: o.get_field("defaultSchemaId"),
            display_name: o.get_field("displayName"),
            document_processing_config: o.get_field("documentProcessingConfig"),
            industry_vertical: o.get_field("industryVertical"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            skip_default_schema_creation: o.get_field("skipDefaultSchemaCreation"),
            solution_types: o.get_field("solutionTypes"),
        }
    }
}
