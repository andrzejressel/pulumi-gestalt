#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkInsightsAnalysisReturnPathComponentVpc {
    /// ARN of the selected Network Insights Analysis.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Name of the filter field. Valid values can be found in the EC2 [`DescribeNetworkInsightsAnalyses`](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeNetworkInsightsAnalyses.html) API Reference.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
