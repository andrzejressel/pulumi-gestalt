#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSalesforce {
    #[builder(into)]
    #[serde(rename = "instanceUrl")]
    pub r#instance_url: Option<String>,
    /// Indicates whether the connector profile applies to a sandbox or production environment.
    #[builder(into)]
    #[serde(rename = "isSandboxEnvironment")]
    pub r#is_sandbox_environment: Option<bool>,
}
