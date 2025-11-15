#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDirectoryVpcSetting {
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Vec<String>,
    /// Identifiers of the subnets for the connector servers (2 subnets in 2 different AZs).
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Vec<String>,
    /// ID of the VPC that the connector is in.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: String,
}
