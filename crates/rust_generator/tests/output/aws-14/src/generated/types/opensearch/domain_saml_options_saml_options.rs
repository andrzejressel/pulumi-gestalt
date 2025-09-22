#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainSamlOptionsSamlOptions {
    /// Whether SAML authentication is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Information from your identity provider.
    #[builder(into)]
    #[serde(rename = "idp")]
    pub r#idp: Option<Box<super::super::types::opensearch::DomainSamlOptionsSamlOptionsIdp>>,
    /// This backend role from the SAML IdP receives full permissions to the cluster, equivalent to a new master user.
    #[builder(into)]
    #[serde(rename = "masterBackendRole")]
    pub r#master_backend_role: Option<String>,
    /// This username from the SAML IdP receives full permissions to the cluster, equivalent to a new master user.
    #[builder(into)]
    #[serde(rename = "masterUserName")]
    pub r#master_user_name: Option<String>,
    /// Element of the SAML assertion to use for backend roles. Default is roles.
    #[builder(into)]
    #[serde(rename = "rolesKey")]
    pub r#roles_key: Option<String>,
    /// Duration of a session in minutes after a user logs in. Default is 60. Maximum value is 1,440.
    #[builder(into)]
    #[serde(rename = "sessionTimeoutMinutes")]
    pub r#session_timeout_minutes: Option<i32>,
    /// Element of the SAML assertion to use for username. Default is NameID.
    #[builder(into)]
    #[serde(rename = "subjectKey")]
    pub r#subject_key: Option<String>,
}
