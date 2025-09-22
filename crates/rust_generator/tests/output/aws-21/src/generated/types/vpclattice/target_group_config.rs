#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TargetGroupConfig {
    /// The health check configuration.
    #[builder(into)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Option<Box<super::super::types::vpclattice::TargetGroupConfigHealthCheck>>,
    /// The type of IP address used for the target group. Valid values: `IPV4` | `IPV6`.
    #[builder(into)]
    #[serde(rename = "ipAddressType")]
    pub r#ip_address_type: Option<String>,
    /// The version of the event structure that the Lambda function receives. Supported only if `type` is `LAMBDA`. Valid Values are `V1` | `V2`.
    #[builder(into)]
    #[serde(rename = "lambdaEventStructureVersion")]
    pub r#lambda_event_structure_version: Option<String>,
    /// The port on which the targets are listening.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// The protocol to use for routing traffic to the targets. Valid Values are `HTTP` | `HTTPS`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    /// The protocol version. Valid Values are `HTTP1` | `HTTP2` | `GRPC`. Default value is `HTTP1`.
    #[builder(into)]
    #[serde(rename = "protocolVersion")]
    pub r#protocol_version: Option<String>,
    /// The ID of the VPC.
    #[builder(into)]
    #[serde(rename = "vpcIdentifier")]
    pub r#vpc_identifier: Option<String>,
}
