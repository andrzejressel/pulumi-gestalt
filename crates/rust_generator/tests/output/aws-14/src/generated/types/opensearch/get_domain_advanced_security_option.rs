#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDomainAdvancedSecurityOption {
    #[builder(into)]
    #[serde(rename = "anonymousAuthEnabled")]
    pub r#anonymous_auth_enabled: bool,
    /// Enabled disabled toggle for off-peak update window
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Whether the internal user database is enabled.
    #[builder(into)]
    #[serde(rename = "internalUserDatabaseEnabled")]
    pub r#internal_user_database_enabled: bool,
}
