#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataCollectionRuleDestinationEventHubDirect {
    /// The resource ID of the Event Hub.
    #[builder(into)]
    #[serde(rename = "eventHubId")]
    pub r#event_hub_id: Box<String>,
    /// Specifies the name of the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
