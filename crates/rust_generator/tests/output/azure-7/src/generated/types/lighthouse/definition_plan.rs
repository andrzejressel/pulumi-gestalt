#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DefinitionPlan {
    /// The plan name of the marketplace offer.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The product code of the plan.
    #[builder(into)]
    #[serde(rename = "product")]
    pub r#product: String,
    /// The publisher ID of the plan.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: String,
    /// The version of the plan.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
