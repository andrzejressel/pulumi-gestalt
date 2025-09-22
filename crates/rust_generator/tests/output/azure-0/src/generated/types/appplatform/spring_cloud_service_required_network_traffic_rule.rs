#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudServiceRequiredNetworkTrafficRule {
    /// The direction of required traffic. Possible values are `Inbound`, `Outbound`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Option<String>,
    /// The FQDN list of required traffic.
    #[builder(into)]
    #[serde(rename = "fqdns")]
    pub r#fqdns: Option<Vec<String>>,
    /// The IP list of required traffic.
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Option<Vec<String>>,
    /// The port of required traffic.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// The protocol of required traffic.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
}
