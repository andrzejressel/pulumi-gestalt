#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLaunchTemplateInstanceMarketOption {
    #[builder(into)]
    #[serde(rename = "marketType")]
    pub r#market_type: String,
    #[builder(into)]
    #[serde(rename = "spotOptions")]
    pub r#spot_options: Vec<super::super::types::ec2::GetLaunchTemplateInstanceMarketOptionSpotOption>,
}
