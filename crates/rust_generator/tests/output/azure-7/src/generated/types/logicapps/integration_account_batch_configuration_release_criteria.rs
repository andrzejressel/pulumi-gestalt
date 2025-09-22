#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IntegrationAccountBatchConfigurationReleaseCriteria {
    /// The batch size in bytes for the Logic App Integration Batch Configuration.
    #[builder(into)]
    #[serde(rename = "batchSize")]
    pub r#batch_size: Option<i32>,
    /// The message count for the Logic App Integration Batch Configuration.
    #[builder(into)]
    #[serde(rename = "messageCount")]
    pub r#message_count: Option<i32>,
    /// A `recurrence` block as documented below.
    #[builder(into)]
    #[serde(rename = "recurrence")]
    pub r#recurrence: Option<Box<super::super::types::logicapps::IntegrationAccountBatchConfigurationReleaseCriteriaRecurrence>>,
}
