/// A FhirStore is a datastore inside a Healthcare dataset that conforms to the FHIR (https://www.hl7.org/fhir/STU3/)
/// standard for Healthcare information exchange
///
///
/// To get more information about FhirStore, see:
///
/// * [API documentation](https://cloud.google.com/healthcare/docs/reference/rest/v1/projects.locations.datasets.fhirStores)
/// * How-to Guides
///     * [Creating a FHIR store](https://cloud.google.com/healthcare/docs/how-tos/fhir)
///
/// ## Example Usage
///
/// ### Healthcare Fhir Store Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:healthcare:FhirStore
///     properties:
///       name: example-fhir-store
///       dataset: ${dataset.id}
///       version: R4
///       complexDataTypeReferenceParsing: DISABLED
///       enableUpdateCreate: false
///       disableReferentialIntegrity: false
///       disableResourceVersioning: false
///       enableHistoryImport: false
///       defaultSearchHandlingStrict: false
///       notificationConfigs:
///         - pubsubTopic: ${topic.id}
///       labels:
///         label1: labelvalue1
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: fhir-notifications
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example-dataset
///       location: us-central1
/// ```
/// ### Healthcare Fhir Store Streaming Config
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:healthcare:FhirStore
///     properties:
///       name: example-fhir-store
///       dataset: ${dataset.id}
///       version: R4
///       enableUpdateCreate: false
///       disableReferentialIntegrity: false
///       disableResourceVersioning: false
///       enableHistoryImport: false
///       labels:
///         label1: labelvalue1
///       streamConfigs:
///         - resourceTypes:
///             - Observation
///           bigqueryDestination:
///             datasetUri: bq://${bqDataset.project}.${bqDataset.datasetId}
///             schemaConfig:
///               recursiveStructureDepth: 3
///               lastUpdatedPartitionConfig:
///                 type: HOUR
///                 expirationMs: 1e+06
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: fhir-notifications
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example-dataset
///       location: us-central1
///   bqDataset:
///     type: gcp:bigquery:Dataset
///     name: bq_dataset
///     properties:
///       datasetId: bq_example_dataset
///       friendlyName: test
///       description: This is a test description
///       location: US
///       deleteContentsOnDestroy: true
/// ```
/// ### Healthcare Fhir Store Notification Configs
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:healthcare:FhirStore
///     properties:
///       name: example-fhir-store
///       dataset: ${dataset.id}
///       version: R4
///       enableUpdateCreate: false
///       disableReferentialIntegrity: false
///       disableResourceVersioning: false
///       enableHistoryImport: false
///       labels:
///         label1: labelvalue1
///       notificationConfigs:
///         - pubsubTopic: ${topic.id}
///           sendFullResource: true
///           sendPreviousResourceOnDelete: true
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: fhir-notifications
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example-dataset
///       location: us-central1
/// ```
///
/// ## Import
///
/// FhirStore can be imported using any of these accepted formats:
///
/// * `{{dataset}}/fhirStores/{{name}}`
///
/// * `{{dataset}}/{{name}}`
///
/// When using the `pulumi import` command, FhirStore can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:healthcare/fhirStore:FhirStore default {{dataset}}/fhirStores/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:healthcare/fhirStore:FhirStore default {{dataset}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fhir_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FhirStoreArgs {
        /// Enable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED by default after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources.
        /// Possible values are: `COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED`, `DISABLED`, `ENABLED`.
        #[builder(into, default)]
        pub complex_data_type_reference_parsing: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        ///
        ///
        /// - - -
        #[builder(into)]
        pub dataset: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If true, overrides the default search behavior for this FHIR store to handling=strict which returns an error for unrecognized search parameters.
        /// If false, uses the FHIR specification default handling=lenient which ignores unrecognized search parameters.
        /// The handling can always be changed from the default on an individual API call by setting the HTTP header Prefer: handling=strict or Prefer: handling=lenient.
        #[builder(into, default)]
        pub default_search_handling_strict: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether to disable referential integrity in this FHIR store. This field is immutable after FHIR store
        /// creation. The default value is false, meaning that the API will enforce referential integrity and fail the
        /// requests that will result in inconsistent state in the FHIR store. When this field is set to true, the API
        /// will skip referential integrity check. Consequently, operations that rely on references, such as
        /// Patient.get$everything, will not return all the results if broken references exist.
        /// ** Changing this property may recreate the FHIR store (removing all data) **
        #[builder(into, default)]
        pub disable_referential_integrity: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether to disable resource versioning for this FHIR store. This field can not be changed after the creation
        /// of FHIR store. If set to false, which is the default behavior, all write operations will cause historical
        /// versions to be recorded automatically. The historical versions can be fetched through the history APIs, but
        /// cannot be updated. If set to true, no historical versions will be kept. The server will send back errors for
        /// attempts to read the historical versions.
        /// ** Changing this property may recreate the FHIR store (removing all data) **
        #[builder(into, default)]
        pub disable_resource_versioning: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether to allow the bulk import API to accept history bundles and directly insert historical resource
        /// versions into the FHIR store. Importing resource histories creates resource interactions that appear to have
        /// occurred in the past, which clients may not want to allow. If set to false, history bundles within an import
        /// will fail with an error.
        /// ** Changing this property may recreate the FHIR store (removing all data) **
        /// ** This property can be changed manually in the Google Cloud Healthcare admin console without recreating the FHIR store **
        #[builder(into, default)]
        pub enable_history_import: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether to allow the ExecuteBundle API to accept history bundles, and directly insert and overwrite historical
        /// resource versions into the FHIR store. If set to false, using history bundles fails with an error.
        #[builder(into, default)]
        pub enable_history_modifications: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether this FHIR store has the updateCreate capability. This determines if the client can use an Update
        /// operation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through
        /// the Create operation and attempts to Update a non-existent resource will return errors. Please treat the audit
        /// logs with appropriate levels of care if client-specified resource IDs contain sensitive data such as patient
        /// identifiers, those IDs will be part of the FHIR resource path recorded in Cloud audit logs and Cloud Pub/Sub
        /// notifications.
        #[builder(into, default)]
        pub enable_update_create: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// User-supplied key-value pairs used to organize FHIR stores.
        /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must
        /// conform to the following PCRE regular expression: [\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}
        /// Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128
        /// bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}
        /// No more than 64 labels can be associated with a given store.
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the FhirStore.
        /// ** Changing this property may recreate the FHIR store (removing all data) **
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// (Optional, Deprecated)
        /// A nested object resource.
        /// Structure is documented below.
        ///
        /// > **Warning:** `notification_config` is deprecated and will be removed in a future major release. Use `notification_configs` instead.
        #[builder(into, default)]
        pub notification_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::healthcare::FhirStoreNotificationConfig>,
        >,
        /// A list of notifcation configs that configure the notification for every resource mutation in this FHIR store.
        /// Structure is documented below.
        #[builder(into, default)]
        pub notification_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::healthcare::FhirStoreNotificationConfig>>,
        >,
        /// A list of streaming configs that configure the destinations of streaming export for every resource mutation in
        /// this FHIR store. Each store is allowed to have up to 10 streaming configs. After a new config is added, the next
        /// resource mutation is streamed to the new location in addition to the existing ones. When a location is removed
        /// from the list, the server stops streaming to that location. Before adding a new config, you must add the required
        /// bigquery.dataEditor role to your project's Cloud Healthcare Service Agent service account. Some lag (typically on
        /// the order of dozens of seconds) is expected before the results show up in the streaming destination.
        /// Structure is documented below.
        #[builder(into, default)]
        pub stream_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::healthcare::FhirStoreStreamConfig>>,
        >,
        /// The FHIR specification version.
        /// Default value is `STU3`.
        /// Possible values are: `DSTU2`, `STU3`, `R4`.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FhirStoreResult {
        /// Enable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED by default after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources.
        /// Possible values are: `COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED`, `DISABLED`, `ENABLED`.
        pub complex_data_type_reference_parsing: pulumi_gestalt_rust::Output<String>,
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        ///
        ///
        /// - - -
        pub dataset: pulumi_gestalt_rust::Output<String>,
        /// If true, overrides the default search behavior for this FHIR store to handling=strict which returns an error for unrecognized search parameters.
        /// If false, uses the FHIR specification default handling=lenient which ignores unrecognized search parameters.
        /// The handling can always be changed from the default on an individual API call by setting the HTTP header Prefer: handling=strict or Prefer: handling=lenient.
        pub default_search_handling_strict: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to disable referential integrity in this FHIR store. This field is immutable after FHIR store
        /// creation. The default value is false, meaning that the API will enforce referential integrity and fail the
        /// requests that will result in inconsistent state in the FHIR store. When this field is set to true, the API
        /// will skip referential integrity check. Consequently, operations that rely on references, such as
        /// Patient.get$everything, will not return all the results if broken references exist.
        /// ** Changing this property may recreate the FHIR store (removing all data) **
        pub disable_referential_integrity: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to disable resource versioning for this FHIR store. This field can not be changed after the creation
        /// of FHIR store. If set to false, which is the default behavior, all write operations will cause historical
        /// versions to be recorded automatically. The historical versions can be fetched through the history APIs, but
        /// cannot be updated. If set to true, no historical versions will be kept. The server will send back errors for
        /// attempts to read the historical versions.
        /// ** Changing this property may recreate the FHIR store (removing all data) **
        pub disable_resource_versioning: pulumi_gestalt_rust::Output<Option<bool>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether to allow the bulk import API to accept history bundles and directly insert historical resource
        /// versions into the FHIR store. Importing resource histories creates resource interactions that appear to have
        /// occurred in the past, which clients may not want to allow. If set to false, history bundles within an import
        /// will fail with an error.
        /// ** Changing this property may recreate the FHIR store (removing all data) **
        /// ** This property can be changed manually in the Google Cloud Healthcare admin console without recreating the FHIR store **
        pub enable_history_import: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether to allow the ExecuteBundle API to accept history bundles, and directly insert and overwrite historical
        /// resource versions into the FHIR store. If set to false, using history bundles fails with an error.
        pub enable_history_modifications: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether this FHIR store has the updateCreate capability. This determines if the client can use an Update
        /// operation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through
        /// the Create operation and attempts to Update a non-existent resource will return errors. Please treat the audit
        /// logs with appropriate levels of care if client-specified resource IDs contain sensitive data such as patient
        /// identifiers, those IDs will be part of the FHIR resource path recorded in Cloud audit logs and Cloud Pub/Sub
        /// notifications.
        pub enable_update_create: pulumi_gestalt_rust::Output<Option<bool>>,
        /// User-supplied key-value pairs used to organize FHIR stores.
        /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must
        /// conform to the following PCRE regular expression: [\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}
        /// Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128
        /// bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}
        /// No more than 64 labels can be associated with a given store.
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the FhirStore.
        /// ** Changing this property may recreate the FHIR store (removing all data) **
        pub name: pulumi_gestalt_rust::Output<String>,
        /// (Optional, Deprecated)
        /// A nested object resource.
        /// Structure is documented below.
        ///
        /// > **Warning:** `notification_config` is deprecated and will be removed in a future major release. Use `notification_configs` instead.
        pub notification_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::healthcare::FhirStoreNotificationConfig>,
        >,
        /// A list of notifcation configs that configure the notification for every resource mutation in this FHIR store.
        /// Structure is documented below.
        pub notification_configs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::healthcare::FhirStoreNotificationConfig>>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The fully qualified name of this dataset
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// A list of streaming configs that configure the destinations of streaming export for every resource mutation in
        /// this FHIR store. Each store is allowed to have up to 10 streaming configs. After a new config is added, the next
        /// resource mutation is streamed to the new location in addition to the existing ones. When a location is removed
        /// from the list, the server stops streaming to that location. Before adding a new config, you must add the required
        /// bigquery.dataEditor role to your project's Cloud Healthcare Service Agent service account. Some lag (typically on
        /// the order of dozens of seconds) is expected before the results show up in the streaming destination.
        /// Structure is documented below.
        pub stream_configs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::healthcare::FhirStoreStreamConfig>>,
        >,
        /// The FHIR specification version.
        /// Default value is `STU3`.
        /// Possible values are: `DSTU2`, `STU3`, `R4`.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FhirStoreArgs,
    ) -> FhirStoreResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let complex_data_type_reference_parsing_binding = args
            .complex_data_type_reference_parsing
            .get_output(context);
        let dataset_binding = args.dataset.get_output(context);
        let default_search_handling_strict_binding = args
            .default_search_handling_strict
            .get_output(context);
        let disable_referential_integrity_binding = args
            .disable_referential_integrity
            .get_output(context);
        let disable_resource_versioning_binding = args
            .disable_resource_versioning
            .get_output(context);
        let enable_history_import_binding = args
            .enable_history_import
            .get_output(context);
        let enable_history_modifications_binding = args
            .enable_history_modifications
            .get_output(context);
        let enable_update_create_binding = args.enable_update_create.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_config_binding = args.notification_config.get_output(context);
        let notification_configs_binding = args.notification_configs.get_output(context);
        let stream_configs_binding = args.stream_configs.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:healthcare/fhirStore:FhirStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "complexDataTypeReferenceParsing".into(),
                    value: &complex_data_type_reference_parsing_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataset".into(),
                    value: &dataset_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultSearchHandlingStrict".into(),
                    value: &default_search_handling_strict_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableReferentialIntegrity".into(),
                    value: &disable_referential_integrity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableResourceVersioning".into(),
                    value: &disable_resource_versioning_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableHistoryImport".into(),
                    value: &enable_history_import_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableHistoryModifications".into(),
                    value: &enable_history_modifications_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableUpdateCreate".into(),
                    value: &enable_update_create_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationConfig".into(),
                    value: &notification_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationConfigs".into(),
                    value: &notification_configs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamConfigs".into(),
                    value: &stream_configs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FhirStoreResult {
            complex_data_type_reference_parsing: o
                .get_field("complexDataTypeReferenceParsing"),
            dataset: o.get_field("dataset"),
            default_search_handling_strict: o.get_field("defaultSearchHandlingStrict"),
            disable_referential_integrity: o.get_field("disableReferentialIntegrity"),
            disable_resource_versioning: o.get_field("disableResourceVersioning"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_history_import: o.get_field("enableHistoryImport"),
            enable_history_modifications: o.get_field("enableHistoryModifications"),
            enable_update_create: o.get_field("enableUpdateCreate"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            notification_config: o.get_field("notificationConfig"),
            notification_configs: o.get_field("notificationConfigs"),
            pulumi_labels: o.get_field("pulumiLabels"),
            self_link: o.get_field("selfLink"),
            stream_configs: o.get_field("streamConfigs"),
            version: o.get_field("version"),
        }
    }
}
