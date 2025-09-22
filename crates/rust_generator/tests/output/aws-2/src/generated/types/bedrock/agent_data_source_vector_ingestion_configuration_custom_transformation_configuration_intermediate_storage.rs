#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationIntermediateStorage {
    /// Configuration block for intermedia S3 storage.
    #[builder(into)]
    #[serde(rename = "s3Location")]
    pub r#s_3_location: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationIntermediateStorageS3Location>>,
}
