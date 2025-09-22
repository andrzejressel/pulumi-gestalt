#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ElasticsearchLogs {
    /// A list of `filtering_tag` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "filteringTags")]
    pub r#filtering_tags: Option<Vec<super::super::types::elasticcloud::ElasticsearchLogsFilteringTag>>,
    /// Specifies if the Azure Activity Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sendActivityLogs")]
    pub r#send_activity_logs: Option<bool>,
    /// Specifies if the AzureAD Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sendAzureadLogs")]
    pub r#send_azuread_logs: Option<bool>,
    /// Specifies if the Azure Subscription Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sendSubscriptionLogs")]
    pub r#send_subscription_logs: Option<bool>,
}
