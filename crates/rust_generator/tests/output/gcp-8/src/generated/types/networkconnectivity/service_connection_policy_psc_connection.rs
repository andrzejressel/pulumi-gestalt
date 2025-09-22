#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceConnectionPolicyPscConnection {
    /// The resource reference of the consumer address.
    #[builder(into)]
    #[serde(rename = "consumerAddress")]
    pub r#consumer_address: Option<String>,
    /// The resource reference of the PSC Forwarding Rule within the consumer VPC.
    #[builder(into)]
    #[serde(rename = "consumerForwardingRule")]
    pub r#consumer_forwarding_rule: Option<String>,
    /// The project where the PSC connection is created.
    #[builder(into)]
    #[serde(rename = "consumerTargetProject")]
    pub r#consumer_target_project: Option<String>,
    /// The most recent error during operating this connection.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "error")]
    pub r#error: Option<Box<super::super::types::networkconnectivity::ServiceConnectionPolicyPscConnectionError>>,
    /// The error info for the latest error during operating this connection.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "errorInfo")]
    pub r#error_info: Option<Box<super::super::types::networkconnectivity::ServiceConnectionPolicyPscConnectionErrorInfo>>,
    /// The error type indicates whether the error is consumer facing, producer
    /// facing or system internal.
    /// Possible values are: `CONNECTION_ERROR_TYPE_UNSPECIFIED`, `ERROR_INTERNAL`, `ERROR_CONSUMER_SIDE`, `ERROR_PRODUCER_SIDE`.
    #[builder(into)]
    #[serde(rename = "errorType")]
    pub r#error_type: Option<String>,
    /// The last Compute Engine operation to setup PSC connection.
    #[builder(into)]
    #[serde(rename = "gceOperation")]
    pub r#gce_operation: Option<String>,
    /// The PSC connection id of the PSC forwarding rule.
    #[builder(into)]
    #[serde(rename = "pscConnectionId")]
    pub r#psc_connection_id: Option<String>,
    /// The state of the PSC connection.
    /// Possible values are: `STATE_UNSPECIFIED`, `ACTIVE`, `CREATING`, `DELETING`, `FAILED`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}
