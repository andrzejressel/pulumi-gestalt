#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StorageLensConfigurationStorageLensConfigurationInclude {
    /// List of S3 bucket ARNs.
    #[builder(into)]
    #[serde(rename = "buckets")]
    pub r#buckets: Option<Vec<String>>,
    /// List of AWS Regions.
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Option<Vec<String>>,
}
