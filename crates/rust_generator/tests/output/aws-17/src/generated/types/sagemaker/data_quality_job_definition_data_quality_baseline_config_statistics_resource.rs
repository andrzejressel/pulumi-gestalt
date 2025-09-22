#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataQualityJobDefinitionDataQualityBaselineConfigStatisticsResource {
    /// The Amazon S3 URI for the statistics resource.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: Option<String>,
}
