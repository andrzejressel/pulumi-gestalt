#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetHubApnsCredential {
    /// The Application Mode which defines which server the APNS Messages should be sent to. Possible values are `Production` and `Sandbox`.
    #[builder(into)]
    #[serde(rename = "applicationMode")]
    pub r#application_mode: String,
    /// The Bundle ID of the iOS/macOS application to send push notifications for, such as `com.org.example`.
    #[builder(into)]
    #[serde(rename = "bundleId")]
    pub r#bundle_id: String,
    /// The Apple Push Notifications Service (APNS) Key.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: String,
    /// The ID of the team the Token.
    #[builder(into)]
    #[serde(rename = "teamId")]
    pub r#team_id: String,
    /// The Push Token associated with the Apple Developer Account.
    #[builder(into)]
    #[serde(rename = "token")]
    pub r#token: String,
}
