#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkgroupEndpoint {
    /// The DNS address of the VPC endpoint.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// The port number on which the cluster accepts incoming connections.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// The VPC endpoint or the Redshift Serverless workgroup. See `VPC Endpoint` below.
    #[builder(into)]
    #[serde(rename = "vpcEndpoints")]
    pub r#vpc_endpoints: Option<Vec<super::super::types::redshiftserverless::WorkgroupEndpointVpcEndpoint>>,
}
