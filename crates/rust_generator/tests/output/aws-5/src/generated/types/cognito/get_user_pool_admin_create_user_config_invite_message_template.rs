#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetUserPoolAdminCreateUserConfigInviteMessageTemplate {
    /// - Email message content.
    #[builder(into)]
    #[serde(rename = "emailMessage")]
    pub r#email_message: String,
    /// - Email message subject.
    #[builder(into)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: String,
    /// - SMS message content.
    #[builder(into)]
    #[serde(rename = "smsMessage")]
    pub r#sms_message: String,
}
