#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointCustomDomainCdnManagedHttps {
    /// The type of HTTPS certificate. Possible values are `Shared` and `Dedicated`.
    #[builder(into)]
    #[serde(rename = "certificateType")]
    pub r#certificate_type: String,
    /// The type of protocol. Possible values are `ServerNameIndication` and `IPBased`.
    #[builder(into)]
    #[serde(rename = "protocolType")]
    pub r#protocol_type: String,
    /// The minimum TLS protocol version that is used for HTTPS. Possible values are `TLS10` (representing TLS 1.0/1.1), `TLS12` (representing TLS 1.2) and `None` (representing no minimums). Defaults to `TLS12`.
    /// 
    /// > **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more.
    #[builder(into)]
    #[serde(rename = "tlsVersion")]
    pub r#tls_version: Option<String>,
}
