#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutomationRuleActionPlaybook {
    /// The ID of the Logic App that defines the playbook's logic.
    #[builder(into)]
    #[serde(rename = "logicAppId")]
    pub r#logic_app_id: String,
    /// The execution order of this action.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: i32,
    /// The ID of the Tenant that owns the playbook.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
}
