#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDomainSettingsDockerSettings {
    /// Indicates whether the domain can access Docker. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into)]
    #[serde(rename = "enableDockerAccess")]
    pub r#enable_docker_access: Option<String>,
    /// The list of Amazon Web Services accounts that are trusted when the domain is created in VPC-only mode.
    #[builder(into)]
    #[serde(rename = "vpcOnlyTrustedAccounts")]
    pub r#vpc_only_trusted_accounts: Option<Vec<String>>,
}
