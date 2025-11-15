#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnServerConfigurationRadius {
    /// One or more `client_root_certificate` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "clientRootCertificates")]
    pub r#client_root_certificates: Option<Vec<super::super::types::network::VpnServerConfigurationRadiusClientRootCertificate>>,
    /// One or more `server_root_certificate` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "serverRootCertificates")]
    pub r#server_root_certificates: Option<Vec<super::super::types::network::VpnServerConfigurationRadiusServerRootCertificate>>,
    /// One or more `server` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "servers")]
    pub r#servers: Option<Vec<super::super::types::network::VpnServerConfigurationRadiusServer>>,
}
