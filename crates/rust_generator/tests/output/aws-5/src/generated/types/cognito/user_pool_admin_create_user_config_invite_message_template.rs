#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPoolAdminCreateUserConfigInviteMessageTemplate {
    /// Message template for email messages. Must contain `{username}` and `{####}` placeholders, for username and temporary password, respectively.
    #[builder(into, default)]
    #[serde(rename = "emailMessage")]
    pub r#email_message: Box<Option<String>>,
    /// Subject line for email messages.
    #[builder(into, default)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: Box<Option<String>>,
    /// Message template for SMS messages. Must contain `{username}` and `{####}` placeholders, for username and temporary password, respectively.
    #[builder(into, default)]
    #[serde(rename = "smsMessage")]
    pub r#sms_message: Box<Option<String>>,
}
