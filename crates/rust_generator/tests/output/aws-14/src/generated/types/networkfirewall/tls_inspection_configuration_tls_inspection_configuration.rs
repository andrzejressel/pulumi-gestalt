#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TlsInspectionConfigurationTlsInspectionConfiguration {
    /// Server certificate configurations that are associated with the TLS configuration. Detailed below.
    #[builder(into)]
    #[serde(rename = "serverCertificateConfiguration")]
    pub r#server_certificate_configuration: Option<Box<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfiguration>>,
}
