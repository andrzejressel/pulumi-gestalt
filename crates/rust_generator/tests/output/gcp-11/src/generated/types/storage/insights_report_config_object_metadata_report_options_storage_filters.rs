#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InsightsReportConfigObjectMetadataReportOptionsStorageFilters {
    /// The filter to use when specifying which bucket to generate inventory reports for.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Option<String>,
}
