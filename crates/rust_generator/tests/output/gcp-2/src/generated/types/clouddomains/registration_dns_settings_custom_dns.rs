#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegistrationDnsSettingsCustomDns {
    /// The list of DS records for this domain, which are used to enable DNSSEC. The domain's DNS provider can provide
    /// the values to set here. If this field is empty, DNSSEC is disabled.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dsRecords")]
    pub r#ds_records: Option<Vec<super::super::types::clouddomains::RegistrationDnsSettingsCustomDnsDsRecord>>,
    /// Required. A list of name servers that store the DNS zone for this domain. Each name server is a domain
    /// name, with Unicode domain names expressed in Punycode format.
    #[builder(into)]
    #[serde(rename = "nameServers")]
    pub r#name_servers: Vec<String>,
}
