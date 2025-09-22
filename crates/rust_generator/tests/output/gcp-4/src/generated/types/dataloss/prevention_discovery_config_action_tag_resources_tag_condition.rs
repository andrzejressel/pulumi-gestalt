#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigActionTagResourcesTagCondition {
    /// Conditions attaching the tag to a resource on its profile having this sensitivity score.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sensitivityScore")]
    pub r#sensitivity_score: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigActionTagResourcesTagConditionSensitivityScore>>,
    /// The tag value to attach to resources.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigActionTagResourcesTagConditionTag>>,
}
