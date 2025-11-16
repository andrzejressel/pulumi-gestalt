#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointAccessVpcEndpoint {
    /// The network interfaces of the endpoint.. See `Network Interface` below.
    #[builder(into)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Option<Vec<super::super::types::redshiftserverless::EndpointAccessVpcEndpointNetworkInterface>>,
    /// The DNS address of the VPC endpoint.
    #[builder(into)]
    #[serde(rename = "vpcEndpointId")]
    pub r#vpc_endpoint_id: Option<String>,
    /// The port that Amazon Redshift Serverless listens on.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Option<String>,
}
