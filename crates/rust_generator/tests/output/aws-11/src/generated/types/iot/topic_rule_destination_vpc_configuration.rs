#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleDestinationVpcConfiguration {
    /// The ARN of a role that has permission to create and attach to elastic network interfaces (ENIs).
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The security groups of the VPC destination.
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Option<Vec<String>>,
    /// The subnet IDs of the VPC destination.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Vec<String>,
    /// The ID of the VPC.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: String,
}
