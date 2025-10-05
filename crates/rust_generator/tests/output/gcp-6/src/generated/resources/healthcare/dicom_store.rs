/// A DicomStore is a datastore inside a Healthcare dataset that conforms to the DICOM
/// (https://www.dicomstandard.org/about/) standard for Healthcare information exchange
///
///
/// To get more information about DicomStore, see:
///
/// * [API documentation](https://cloud.google.com/healthcare/docs/reference/rest/v1/projects.locations.datasets.dicomStores)
/// * How-to Guides
///     * [Creating a DICOM store](https://cloud.google.com/healthcare/docs/how-tos/dicom)
///
/// ## Example Usage
///
/// ### Healthcare Dicom Store Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:healthcare:DicomStore
///     properties:
///       name: example-dicom-store
///       dataset: ${dataset.id}
///       notificationConfig:
///         pubsubTopic: ${topic.id}
///       labels:
///         label1: labelvalue1
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: dicom-notifications
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example-dataset
///       location: us-central1
/// ```
/// ### Healthcare Dicom Store Bq Stream
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:healthcare:DicomStore
///     properties:
///       name: example-dicom-store
///       dataset: ${dataset.id}
///       notificationConfig:
///         pubsubTopic: ${topic.id}
///         sendForBulkImport: true
///       labels:
///         label1: labelvalue1
///       streamConfigs:
///         - bigqueryDestination:
///             tableUri: bq://${bqDataset.project}.${bqDataset.datasetId}.${bqTable.tableId}
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: dicom-notifications
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example-dataset
///       location: us-central1
///   bqDataset:
///     type: gcp:bigquery:Dataset
///     name: bq_dataset
///     properties:
///       datasetId: dicom_bq_ds
///       friendlyName: test
///       description: This is a test description
///       location: US
///       deleteContentsOnDestroy: true
///   bqTable:
///     type: gcp:bigquery:Table
///     name: bq_table
///     properties:
///       deletionProtection: false
///       datasetId: ${bqDataset.datasetId}
///       tableId: dicom_bq_tb
/// ```
///
/// ## Import
///
/// DicomStore can be imported using any of these accepted formats:
///
/// * `{{dataset}}/dicomStores/{{name}}`
///
/// * `{{dataset}}/{{name}}`
///
/// When using the `pulumi import` command, DicomStore can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:healthcare/dicomStore:DicomStore default {{dataset}}/dicomStores/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:healthcare/dicomStore:DicomStore default {{dataset}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dicom_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DicomStoreArgs {
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        ///
        ///
        /// - - -
        #[builder(into)]
        pub dataset: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User-supplied key-value pairs used to organize DICOM stores.
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
        /// The resource name for the DicomStore.
        /// ** Changing this property may recreate the Dicom store (removing all data) **
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub notification_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::healthcare::DicomStoreNotificationConfig>,
        >,
        /// To enable streaming to BigQuery, configure the streamConfigs object in your DICOM store.
        /// streamConfigs is an array, so you can specify multiple BigQuery destinations. You can stream metadata from a single DICOM store to up to five BigQuery tables in a BigQuery dataset.
        /// Structure is documented below.
        #[builder(into, default)]
        pub stream_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::healthcare::DicomStoreStreamConfig>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DicomStoreResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        ///
        ///
        /// - - -
        pub dataset: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// User-supplied key-value pairs used to organize DICOM stores.
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
        /// The resource name for the DicomStore.
        /// ** Changing this property may recreate the Dicom store (removing all data) **
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A nested object resource.
        /// Structure is documented below.
        pub notification_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::healthcare::DicomStoreNotificationConfig>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The fully qualified name of this dataset
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// To enable streaming to BigQuery, configure the streamConfigs object in your DICOM store.
        /// streamConfigs is an array, so you can specify multiple BigQuery destinations. You can stream metadata from a single DICOM store to up to five BigQuery tables in a BigQuery dataset.
        /// Structure is documented below.
        pub stream_configs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::healthcare::DicomStoreStreamConfig>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DicomStoreArgs,
    ) -> DicomStoreResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dataset_binding = args.dataset.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_config_binding = args.notification_config.get_output(context);
        let stream_configs_binding = args.stream_configs.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:healthcare/dicomStore:DicomStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataset".into(),
                    value: &dataset_binding.drop_type(),
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
                    name: "streamConfigs".into(),
                    value: &stream_configs_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DicomStoreResult {
            id: o.get_field("id"),
            dataset: o.get_field("dataset"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            notification_config: o.get_field("notificationConfig"),
            pulumi_labels: o.get_field("pulumiLabels"),
            self_link: o.get_field("selfLink"),
            stream_configs: o.get_field("streamConfigs"),
        }
    }
}
