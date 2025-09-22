#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetReplicationGroupLogDeliveryConfiguration {
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: String,
    #[builder(into)]
    #[serde(rename = "destinationType")]
    pub r#destination_type: String,
    #[builder(into)]
    #[serde(rename = "logFormat")]
    pub r#log_format: String,
    #[builder(into)]
    #[serde(rename = "logType")]
    pub r#log_type: String,
}
