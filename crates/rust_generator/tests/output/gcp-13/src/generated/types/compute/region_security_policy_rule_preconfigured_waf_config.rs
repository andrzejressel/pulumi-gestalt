#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionSecurityPolicyRulePreconfiguredWafConfig {
    /// An exclusion to apply during preconfigured WAF evaluation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "exclusions")]
    pub r#exclusions: Option<Vec<super::super::types::compute::RegionSecurityPolicyRulePreconfiguredWafConfigExclusion>>,
}
