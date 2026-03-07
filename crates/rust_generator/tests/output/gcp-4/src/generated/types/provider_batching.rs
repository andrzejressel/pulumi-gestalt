#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderBatching {
    #[builder(into)]
    #[serde(rename = "enableBatching")]
    pub r#enable_batching: Option<bool>,
    #[builder(into)]
    #[serde(rename = "sendAfter")]
    pub r#send_after: Option<String>,
}
