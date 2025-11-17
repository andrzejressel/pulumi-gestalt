#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchTemplatePlacement {
    #[builder(into)]
    #[serde(rename = "affinity")]
    pub r#affinity: String,
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: String,
    #[builder(into)]
    #[serde(rename = "groupName")]
    pub r#group_name: String,
    #[builder(into)]
    #[serde(rename = "hostId")]
    pub r#host_id: String,
    #[builder(into)]
    #[serde(rename = "hostResourceGroupArn")]
    pub r#host_resource_group_arn: String,
    #[builder(into)]
    #[serde(rename = "partitionNumber")]
    pub r#partition_number: i32,
    #[builder(into)]
    #[serde(rename = "spreadDomain")]
    pub r#spread_domain: String,
    #[builder(into)]
    #[serde(rename = "tenancy")]
    pub r#tenancy: String,
}
