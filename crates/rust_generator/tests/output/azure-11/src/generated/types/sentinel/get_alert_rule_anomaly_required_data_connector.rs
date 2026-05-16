#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAlertRuleAnomalyRequiredDataConnector {
    /// The ID of the required Data Connector.
    #[builder(into)]
    #[serde(rename = "connectorId")]
    pub r#connector_id: String,
    /// A list of data types of the required Data Connector.
    #[builder(into)]
    #[serde(rename = "dataTypes")]
    pub r#data_types: Vec<String>,
}
