#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionDefaultCacheBehaviorForwardedValuesCookies {
    /// Whether you want CloudFront to forward cookies to the origin that is associated with this cache behavior. You can specify `all`, `none` or `whitelist`. If `whitelist`, you must include the subsequent `whitelisted_names`.
    #[builder(into)]
    #[serde(rename = "forward")]
    pub r#forward: String,
    /// If you have specified `whitelist` to `forward`, the whitelisted cookies that you want CloudFront to forward to your origin.
    #[builder(into)]
    #[serde(rename = "whitelistedNames")]
    pub r#whitelisted_names: Option<Vec<String>>,
}
