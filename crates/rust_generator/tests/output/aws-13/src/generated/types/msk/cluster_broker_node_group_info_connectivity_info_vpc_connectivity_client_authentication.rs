#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication {
    /// Configuration block for specifying SASL client authentication. See below.
    #[builder(into)]
    #[serde(rename = "sasl")]
    pub r#sasl: Box<Option<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl>>,
    /// Configuration block for specifying TLS client authentication. See below.
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Option<bool>,
}
