#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetActionDefinition {
    /// The AWS Identity and Access Management (IAM) action definition details. See IAM Action Definition.
    #[builder(into)]
    #[serde(rename = "iamActionDefinition")]
    pub r#iam_action_definition: Option<Box<super::super::types::budgets::BudgetActionDefinitionIamActionDefinition>>,
    /// The service control policies (SCPs) action definition details. See SCP Action Definition.
    #[builder(into)]
    #[serde(rename = "scpActionDefinition")]
    pub r#scp_action_definition: Option<Box<super::super::types::budgets::BudgetActionDefinitionScpActionDefinition>>,
    /// The AWS Systems Manager (SSM) action definition details. See SSM Action Definition.
    #[builder(into)]
    #[serde(rename = "ssmActionDefinition")]
    pub r#ssm_action_definition: Option<Box<super::super::types::budgets::BudgetActionDefinitionSsmActionDefinition>>,
}
