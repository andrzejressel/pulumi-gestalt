#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServerlessClusterVpcConfig {
    /// Specifies up to five security groups that control inbound and outbound traffic for the serverless cluster.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Option<Vec<String>>,
    /// A list of subnets in at least two different Availability Zones that host your client applications.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Vec<String>,
}
