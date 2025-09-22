#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertPolicyAlertStrategy {
    /// If an alert policy that was active has no data for this long, any open incidents will close.
    #[builder(into)]
    #[serde(rename = "autoClose")]
    pub r#auto_close: Option<String>,
    /// Control over how the notification channels in `notification_channels`
    /// are notified when this alert fires, on a per-channel basis.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "notificationChannelStrategies")]
    pub r#notification_channel_strategies: Option<Vec<super::super::types::monitoring::AlertPolicyAlertStrategyNotificationChannelStrategy>>,
    /// Control when notifications will be sent out.
    /// Each value may be one of: `NOTIFICATION_PROMPT_UNSPECIFIED`, `OPENED`, `CLOSED`.
    #[builder(into)]
    #[serde(rename = "notificationPrompts")]
    pub r#notification_prompts: Option<Vec<String>>,
    /// Required for alert policies with a LogMatch condition.
    /// This limit is not implemented for alert policies that are not log-based.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "notificationRateLimit")]
    pub r#notification_rate_limit: Box<Option<super::super::types::monitoring::AlertPolicyAlertStrategyNotificationRateLimit>>,
}
