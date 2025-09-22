#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AgentDataSourceVectorIngestionConfigurationCustomTransformationConfiguration {
    /// The intermediate storage for custom transformation.
    #[builder(into)]
    #[serde(rename = "intermediateStorage")]
    pub r#intermediate_storage: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationIntermediateStorage>>,
    /// A custom processing step for documents moving through the data source ingestion pipeline.
    #[builder(into)]
    #[serde(rename = "transformation")]
    pub r#transformation: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationTransformation>>,
}
