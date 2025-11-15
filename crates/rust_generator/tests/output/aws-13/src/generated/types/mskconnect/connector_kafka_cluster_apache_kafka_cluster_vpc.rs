#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorKafkaClusterApacheKafkaClusterVpc {
    /// The security groups for the connector.
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Vec<String>,
    /// The subnets for the connector.
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Vec<String>,
}
