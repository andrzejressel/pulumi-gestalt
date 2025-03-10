#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReplicatorKafkaCluster {
    /// Details of an Amazon MSK cluster.
    #[builder(into)]
    #[serde(rename = "amazonMskCluster")]
    pub r#amazon_msk_cluster: Box<super::super::types::msk::ReplicatorKafkaClusterAmazonMskCluster>,
    /// Details of an Amazon VPC which has network connectivity to the Apache Kafka cluster.
    #[builder(into)]
    #[serde(rename = "vpcConfig")]
    pub r#vpc_config: Box<super::super::types::msk::ReplicatorKafkaClusterVpcConfig>,
}
