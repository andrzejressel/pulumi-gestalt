#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataQualityJobDefinitionDataQualityJobInputBatchTransformInputDatasetFormat {
    /// The CSV dataset used in the monitoring job. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "csv")]
    pub r#csv: Option<Box<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobInputBatchTransformInputDatasetFormatCsv>>,
    /// The JSON dataset used in the monitoring job. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "json")]
    pub r#json: Option<Box<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobInputBatchTransformInputDatasetFormatJson>>,
}
