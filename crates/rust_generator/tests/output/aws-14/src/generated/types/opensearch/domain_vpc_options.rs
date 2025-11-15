#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainVpcOptions {
    /// If the domain was created inside a VPC, the names of the availability zones the configured `subnet_ids` were created inside.
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Option<Vec<String>>,
    /// List of VPC Security Group IDs to be applied to the OpenSearch domain endpoints. If omitted, the default Security Group for the VPC will be used.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Option<Vec<String>>,
    /// List of VPC Subnet IDs for the OpenSearch domain endpoints to be created in.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Option<Vec<String>>,
    /// If the domain was created inside a VPC, the ID of the VPC.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Option<String>,
}
