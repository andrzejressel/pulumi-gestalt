#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketWebsiteConfigurationV2RoutingRule {
    /// Configuration block for describing a condition that must be met for the specified redirect to apply. See below.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<Box<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRuleCondition>>,
    /// Configuration block for redirect information. See below.
    #[builder(into)]
    #[serde(rename = "redirect")]
    pub r#redirect: Box<super::super::types::s3::BucketWebsiteConfigurationV2RoutingRuleRedirect>,
}
