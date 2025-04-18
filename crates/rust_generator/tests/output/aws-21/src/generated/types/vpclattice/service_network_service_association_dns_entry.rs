#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceNetworkServiceAssociationDnsEntry {
    /// The domain name of the service.
    #[builder(into, default)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<Option<String>>,
    /// The ID of the hosted zone.
    #[builder(into, default)]
    #[serde(rename = "hostedZoneId")]
    pub r#hosted_zone_id: Box<Option<String>>,
}
