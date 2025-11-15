#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchTemplatePlacement {
    /// The affinity setting for an instance on a Dedicated Host.
    #[builder(into)]
    #[serde(rename = "affinity")]
    pub r#affinity: Option<String>,
    /// The Availability Zone for the instance.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Option<String>,
    /// The name of the placement group for the instance.
    #[builder(into)]
    #[serde(rename = "groupName")]
    pub r#group_name: Option<String>,
    /// The ID of the Dedicated Host for the instance.
    #[builder(into)]
    #[serde(rename = "hostId")]
    pub r#host_id: Option<String>,
    /// The ARN of the Host Resource Group in which to launch instances.
    #[builder(into)]
    #[serde(rename = "hostResourceGroupArn")]
    pub r#host_resource_group_arn: Option<String>,
    /// The number of the partition the instance should launch in. Valid only if the placement group strategy is set to partition.
    #[builder(into)]
    #[serde(rename = "partitionNumber")]
    pub r#partition_number: Option<i32>,
    /// Reserved for future use.
    #[builder(into)]
    #[serde(rename = "spreadDomain")]
    pub r#spread_domain: Option<String>,
    /// The tenancy of the instance (if the instance is running in a VPC). Can be `default`, `dedicated`, or `host`.
    #[builder(into)]
    #[serde(rename = "tenancy")]
    pub r#tenancy: Option<String>,
}
