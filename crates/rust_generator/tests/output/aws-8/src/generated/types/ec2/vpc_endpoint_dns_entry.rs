#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpcEndpointDnsEntry {
    /// The DNS name.
    #[builder(into)]
    #[serde(rename = "dnsName")]
    pub r#dns_name: Option<String>,
    /// The ID of the private hosted zone.
    #[builder(into)]
    #[serde(rename = "hostedZoneId")]
    pub r#hosted_zone_id: Option<String>,
}
