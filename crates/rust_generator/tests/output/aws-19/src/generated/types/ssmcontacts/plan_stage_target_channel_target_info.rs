#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PlanStageTargetChannelTargetInfo {
    /// The Amazon Resource Name (ARN) of the contact channel.
    #[builder(into)]
    #[serde(rename = "contactChannelId")]
    pub r#contact_channel_id: String,
    /// The number of minutes to wait before retrying to send engagement if the engagement initially failed.
    #[builder(into)]
    #[serde(rename = "retryIntervalInMinutes")]
    pub r#retry_interval_in_minutes: Option<i32>,
}
