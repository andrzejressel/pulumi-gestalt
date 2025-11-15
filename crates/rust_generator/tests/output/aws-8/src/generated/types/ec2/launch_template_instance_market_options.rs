#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchTemplateInstanceMarketOptions {
    /// The market type. Can be `spot`.
    #[builder(into)]
    #[serde(rename = "marketType")]
    pub r#market_type: Option<String>,
    /// The options for [Spot Instance](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-spot-instances.html)
    #[builder(into)]
    #[serde(rename = "spotOptions")]
    pub r#spot_options: Option<Box<super::super::types::ec2::LaunchTemplateInstanceMarketOptionsSpotOptions>>,
}
