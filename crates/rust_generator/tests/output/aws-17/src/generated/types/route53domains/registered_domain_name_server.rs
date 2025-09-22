#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegisteredDomainNameServer {
    /// Glue IP addresses of a name server. The list can contain only one IPv4 and one IPv6 address.
    #[builder(into)]
    #[serde(rename = "glueIps")]
    pub r#glue_ips: Option<Vec<String>>,
    /// The fully qualified host name of the name server.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
