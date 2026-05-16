#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesSapoDataParallelismConfig {
    /// he maximum number of records that Amazon AppFlow receives in each page of the response from your SAP application.
    #[builder(into)]
    #[serde(rename = "maxPageSize")]
    pub r#max_page_size: i32,
}
