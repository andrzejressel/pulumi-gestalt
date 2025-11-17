#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthConfigDecryptedCredentialUsernameAndPassword {
    /// Password to be used.
    /// 
    /// <a name="nested_oauth2_authorization_code"></a>The `oauth2_authorization_code` block supports:
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// Username to be used.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
