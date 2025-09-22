#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServerEndpointDetails {
    /// A list of address allocation IDs that are required to attach an Elastic IP address to your SFTP server's endpoint. This property can only be used when `endpoint_type` is set to `VPC`.
    #[builder(into)]
    #[serde(rename = "addressAllocationIds")]
    pub r#address_allocation_ids: Option<Vec<String>>,
    /// A list of security groups IDs that are available to attach to your server's endpoint. If no security groups are specified, the VPC's default security groups are automatically assigned to your endpoint. This property can only be used when `endpoint_type` is set to `VPC`.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Option<Vec<String>>,
    /// A list of subnet IDs that are required to host your SFTP server endpoint in your VPC. This property can only be used when `endpoint_type` is set to `VPC`.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Option<Vec<String>>,
    /// The ID of the VPC endpoint. This property can only be used when `endpoint_type` is set to `VPC_ENDPOINT`
    #[builder(into)]
    #[serde(rename = "vpcEndpointId")]
    pub r#vpc_endpoint_id: Option<String>,
    /// The VPC ID of the virtual private cloud in which the SFTP server's endpoint will be hosted. This property can only be used when `endpoint_type` is set to `VPC`.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Option<String>,
}
