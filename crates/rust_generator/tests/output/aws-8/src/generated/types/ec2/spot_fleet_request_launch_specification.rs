#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpotFleetRequestLaunchSpecification {
    #[builder(into)]
    #[serde(rename = "ami")]
    pub r#ami: String,
    #[builder(into)]
    #[serde(rename = "associatePublicIpAddress")]
    pub r#associate_public_ip_address: Option<bool>,
    /// The availability zone in which to place the request.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Option<String>,
    #[builder(into)]
    #[serde(rename = "ebsBlockDevices")]
    pub r#ebs_block_devices: Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchSpecificationEbsBlockDevice>>,
    #[builder(into)]
    #[serde(rename = "ebsOptimized")]
    pub r#ebs_optimized: Option<bool>,
    #[builder(into)]
    #[serde(rename = "ephemeralBlockDevices")]
    pub r#ephemeral_block_devices: Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchSpecificationEphemeralBlockDevice>>,
    #[builder(into)]
    #[serde(rename = "iamInstanceProfile")]
    pub r#iam_instance_profile: Option<String>,
    #[builder(into)]
    #[serde(rename = "iamInstanceProfileArn")]
    pub r#iam_instance_profile_arn: Option<String>,
    /// The type of instance to request.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: String,
    #[builder(into)]
    #[serde(rename = "keyName")]
    pub r#key_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "monitoring")]
    pub r#monitoring: Option<bool>,
    #[builder(into)]
    #[serde(rename = "placementGroup")]
    pub r#placement_group: Option<String>,
    #[builder(into)]
    #[serde(rename = "placementTenancy")]
    pub r#placement_tenancy: Option<String>,
    #[builder(into)]
    #[serde(rename = "rootBlockDevices")]
    pub r#root_block_devices: Option<Vec<super::super::types::ec2::SpotFleetRequestLaunchSpecificationRootBlockDevice>>,
    /// The maximum bid price per unit hour.
    #[builder(into)]
    #[serde(rename = "spotPrice")]
    pub r#spot_price: Option<String>,
    /// The subnet in which to launch the requested instance.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
    /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "userData")]
    pub r#user_data: Option<String>,
    #[builder(into)]
    #[serde(rename = "vpcSecurityGroupIds")]
    pub r#vpc_security_group_ids: Option<Vec<String>>,
    /// The capacity added to the fleet by a fulfilled request.
    #[builder(into)]
    #[serde(rename = "weightedCapacity")]
    pub r#weighted_capacity: Option<String>,
}
