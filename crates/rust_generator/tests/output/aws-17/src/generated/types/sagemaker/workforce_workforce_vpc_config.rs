#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkforceWorkforceVpcConfig {
    /// The VPC security group IDs. The security groups must be for the same VPC as specified in the subnet.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Option<Vec<String>>,
    /// The ID of the subnets in the VPC that you want to connect.
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Option<Vec<String>>,
    /// The IDs for the VPC service endpoints of your VPC workforce.
    #[builder(into)]
    #[serde(rename = "vpcEndpointId")]
    pub r#vpc_endpoint_id: Option<String>,
    /// The ID of the VPC that the workforce uses for communication.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Option<String>,
}
