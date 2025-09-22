#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EntitlementAdditionalNotificationTargets {
    /// Optional. Additional email addresses to be notified when a principal(requester) is granted access.
    #[builder(into)]
    #[serde(rename = "adminEmailRecipients")]
    pub r#admin_email_recipients: Option<Vec<String>>,
    /// Optional. Additional email address to be notified about an eligible entitlement.
    #[builder(into)]
    #[serde(rename = "requesterEmailRecipients")]
    pub r#requester_email_recipients: Option<Vec<String>>,
}
