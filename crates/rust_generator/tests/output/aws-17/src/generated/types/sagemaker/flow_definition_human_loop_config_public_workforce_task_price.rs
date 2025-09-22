#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowDefinitionHumanLoopConfigPublicWorkforceTaskPrice {
    /// Defines the amount of money paid to an Amazon Mechanical Turk worker in United States dollars. See Amount In Usd details below.
    #[builder(into)]
    #[serde(rename = "amountInUsd")]
    pub r#amount_in_usd: Option<Box<super::super::types::sagemaker::FlowDefinitionHumanLoopConfigPublicWorkforceTaskPriceAmountInUsd>>,
}
