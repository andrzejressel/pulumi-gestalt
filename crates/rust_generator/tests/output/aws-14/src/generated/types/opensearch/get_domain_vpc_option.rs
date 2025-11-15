#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDomainVpcOption {
    /// Availability zones used by the domain.
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Vec<String>,
    /// Security groups used by the domain.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Vec<String>,
    /// Subnets used by the domain.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Vec<String>,
    /// VPC used by the domain.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: String,
}
