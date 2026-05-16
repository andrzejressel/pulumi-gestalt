#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication {
    /// Configuration block for specifying SASL client authentication. See below.
    #[builder(into)]
    #[serde(rename = "sasl")]
    pub r#sasl: Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl>>,
    /// Configuration block for specifying TLS client authentication. See below.
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Option<bool>,
}
