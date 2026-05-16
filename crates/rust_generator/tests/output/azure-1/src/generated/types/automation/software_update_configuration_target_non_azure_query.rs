#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
