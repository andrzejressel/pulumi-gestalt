#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NotificationChannelSensitiveLabels {
    /// An authorization token for a notification channel. Channel types that support this field include: slack
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "authToken")]
    pub r#auth_token: Option<String>,
    /// An password for a notification channel. Channel types that support this field include: webhook_basicauth
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// An servicekey token for a notification channel. Channel types that support this field include: pagerduty
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "serviceKey")]
    pub r#service_key: Option<String>,
}
