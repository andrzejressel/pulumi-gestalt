#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventSubscriptionWebhookEndpoint {
    /// The Azure Active Directory Application ID or URI to get the access token that will be included as the bearer token in delivery requests.
    #[builder(into)]
    #[serde(rename = "activeDirectoryAppIdOrUri")]
    pub r#active_directory_app_id_or_uri: Option<String>,
    /// The Azure Active Directory Tenant ID to get the access token that will be included as the bearer token in delivery requests.
    #[builder(into)]
    #[serde(rename = "activeDirectoryTenantId")]
    pub r#active_directory_tenant_id: Option<String>,
    /// The base url of the webhook where the Event Subscription will receive events.
    #[builder(into)]
    #[serde(rename = "baseUrl")]
    pub r#base_url: Option<String>,
    /// Maximum number of events per batch.
    #[builder(into)]
    #[serde(rename = "maxEventsPerBatch")]
    pub r#max_events_per_batch: Option<i32>,
    /// Preferred batch size in Kilobytes.
    #[builder(into)]
    #[serde(rename = "preferredBatchSizeInKilobytes")]
    pub r#preferred_batch_size_in_kilobytes: Option<i32>,
    /// Specifies the url of the webhook where the Event Subscription will receive events.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}
