#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkInsightsAnalysisReturnPathComponentSecurityGroupRule {
    #[builder(into)]
    #[serde(rename = "cidr")]
    pub r#cidr: Option<String>,
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Option<String>,
    #[builder(into)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentSecurityGroupRulePortRange>>,
    #[builder(into)]
    #[serde(rename = "prefixListId")]
    pub r#prefix_list_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    #[builder(into)]
    #[serde(rename = "securityGroupId")]
    pub r#security_group_id: Option<String>,
}
