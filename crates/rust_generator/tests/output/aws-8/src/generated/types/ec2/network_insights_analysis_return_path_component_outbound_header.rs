#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkInsightsAnalysisReturnPathComponentOutboundHeader {
    #[builder(into)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "destinationPortRanges")]
    pub r#destination_port_ranges: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentOutboundHeaderDestinationPortRange>>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    #[builder(into)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "sourcePortRanges")]
    pub r#source_port_ranges: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentOutboundHeaderSourcePortRange>>,
}
