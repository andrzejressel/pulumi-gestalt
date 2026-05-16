#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourceSetResourceDnsTargetResourceTargetResource {
    /// NLB resource a DNS Target Resource points to. Required if `r53_resource` is not set.
    #[builder(into)]
    #[serde(rename = "nlbResource")]
    pub r#nlb_resource: Option<Box<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResourceTargetResourceNlbResource>>,
    /// Route53 resource a DNS Target Resource record points to.
    #[builder(into)]
    #[serde(rename = "r53Resource")]
    pub r#r_53_resource: Option<Box<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResourceTargetResourceR53Resource>>,
}
