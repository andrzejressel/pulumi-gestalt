#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainScalingParameters {
    /// The instance type that you want to preconfigure for your domain. See the [AWS documentation](https://docs.aws.amazon.com/cloudsearch/latest/developerguide/API_ScalingParameters.html) for valid values.
    #[builder(into)]
    #[serde(rename = "desiredInstanceType")]
    pub r#desired_instance_type: Option<String>,
    /// The number of partitions you want to preconfigure for your domain. Only valid when you select `search.2xlarge` as the instance type.
    #[builder(into)]
    #[serde(rename = "desiredPartitionCount")]
    pub r#desired_partition_count: Option<i32>,
    /// The number of replicas you want to preconfigure for each index partition.
    #[builder(into)]
    #[serde(rename = "desiredReplicationCount")]
    pub r#desired_replication_count: Option<i32>,
}
