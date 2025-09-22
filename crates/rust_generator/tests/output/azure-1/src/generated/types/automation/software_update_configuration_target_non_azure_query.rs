#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SoftwareUpdateConfigurationTargetNonAzureQuery {
    /// Specifies the Log Analytics save search name.
    #[builder(into)]
    #[serde(rename = "functionAlias")]
    pub r#function_alias: Option<String>,
    /// The workspace id for Log Analytics in which the saved search in.
    #[builder(into)]
    #[serde(rename = "workspaceId")]
    pub r#workspace_id: Option<String>,
}
