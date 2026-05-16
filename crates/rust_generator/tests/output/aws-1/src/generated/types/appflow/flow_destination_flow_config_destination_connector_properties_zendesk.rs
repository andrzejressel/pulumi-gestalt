#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesZendesk {
    #[builder(into)]
    #[serde(rename = "errorHandlingConfig")]
    pub r#error_handling_config: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesZendeskErrorHandlingConfig>>,
    #[builder(into)]
    #[serde(rename = "idFieldNames")]
    pub r#id_field_names: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: String,
    #[builder(into)]
    #[serde(rename = "writeOperationType")]
    pub r#write_operation_type: Option<String>,
}
