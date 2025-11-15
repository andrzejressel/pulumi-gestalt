#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPlanRuleCopyAction {
    #[builder(into)]
    #[serde(rename = "destinationVaultArn")]
    pub r#destination_vault_arn: String,
    #[builder(into)]
    #[serde(rename = "lifecycles")]
    pub r#lifecycles: Vec<super::super::types::backup::GetPlanRuleCopyActionLifecycle>,
}
