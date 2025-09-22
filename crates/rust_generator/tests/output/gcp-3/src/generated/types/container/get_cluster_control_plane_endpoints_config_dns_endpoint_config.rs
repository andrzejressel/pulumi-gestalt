#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterControlPlaneEndpointsConfigDnsEndpointConfig {
    /// Controls whether user traffic is allowed over this endpoint. Note that GCP-managed services may still use the endpoint even if this is false.
    #[builder(into)]
    #[serde(rename = "allowExternalTraffic")]
    pub r#allow_external_traffic: bool,
    /// The cluster's DNS endpoint.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: String,
}
