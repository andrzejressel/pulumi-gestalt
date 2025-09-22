#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DicomStoreStreamConfigBigqueryDestination {
    /// a fully qualified BigQuery table URI where DICOM instance metadata will be streamed.
    #[builder(into)]
    #[serde(rename = "tableUri")]
    pub r#table_uri: String,
}
