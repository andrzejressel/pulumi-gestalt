#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionDefaultCacheBehaviorFunctionAssociation {
    /// Specific event to trigger this function. Valid values: `viewer-request` or `viewer-response`.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: String,
    /// ARN of the CloudFront function.
    #[builder(into)]
    #[serde(rename = "functionArn")]
    pub r#function_arn: String,
}
