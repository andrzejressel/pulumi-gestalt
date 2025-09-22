#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IntegrationRuntimeSsisPipelineExternalComputeScale {
    /// Specifies the number of the external nodes, which should be greater than `0` and less than `11`.
    #[builder(into)]
    #[serde(rename = "numberOfExternalNodes")]
    pub r#number_of_external_nodes: Option<i32>,
    /// Specifies the number of the pipeline nodes, which should be greater than `0` and less than `11`.
    #[builder(into)]
    #[serde(rename = "numberOfPipelineNodes")]
    pub r#number_of_pipeline_nodes: Option<i32>,
    /// Specifies the time to live (in minutes) setting of integration runtime which will execute copy activity. Possible values are at least `5`.
    #[builder(into)]
    #[serde(rename = "timeToLive")]
    pub r#time_to_live: Option<i32>,
}
