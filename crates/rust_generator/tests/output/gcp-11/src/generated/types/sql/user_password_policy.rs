#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPasswordPolicy {
    /// Number of failed attempts allowed before the user get locked.
    #[builder(into)]
    #[serde(rename = "allowedFailedAttempts")]
    pub r#allowed_failed_attempts: Option<i32>,
    /// If true, the check that will lock user after too many failed login attempts will be enabled.
    #[builder(into)]
    #[serde(rename = "enableFailedAttemptsCheck")]
    pub r#enable_failed_attempts_check: Option<bool>,
    /// If true, the user must specify the current password before changing the password. This flag is supported only for MySQL.
    #[builder(into)]
    #[serde(rename = "enablePasswordVerification")]
    pub r#enable_password_verification: Option<bool>,
    /// Password expiration duration with one week grace period.
    #[builder(into)]
    #[serde(rename = "passwordExpirationDuration")]
    pub r#password_expiration_duration: Option<String>,
    #[builder(into)]
    #[serde(rename = "statuses")]
    pub r#statuses: Option<Vec<super::super::types::sql::UserPasswordPolicyStatus>>,
}
