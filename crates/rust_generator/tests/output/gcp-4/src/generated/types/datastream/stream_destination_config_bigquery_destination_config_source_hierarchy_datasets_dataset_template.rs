#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasetsDatasetTemplate {
    /// If supplied, every created dataset will have its name prefixed by the provided value.
    /// The prefix and name will be separated by an underscore. i.e. _.
    #[builder(into)]
    #[serde(rename = "datasetIdPrefix")]
    pub r#dataset_id_prefix: Option<String>,
    /// Describes the Cloud KMS encryption key that will be used to protect destination BigQuery
    /// table. The BigQuery Service Account associated with your project requires access to this
    /// encryption key. i.e. projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{cryptoKey}.
    /// See https://cloud.google.com/bigquery/docs/customer-managed-encryption for more information.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Option<String>,
    /// The geographic location where the dataset should reside.
    /// See https://cloud.google.com/bigquery/docs/locations for supported locations.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
}
