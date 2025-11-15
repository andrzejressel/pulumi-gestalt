#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetActionGroupLogicAppReceiver {
    /// The callback url where HTTP request sent to.
    #[builder(into)]
    #[serde(rename = "callbackUrl")]
    pub r#callback_url: String,
    /// Specifies the name of the Action Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The Azure resource ID of the logic app.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: String,
    /// Indicates whether to use common alert schema.
    #[builder(into)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: bool,
}
