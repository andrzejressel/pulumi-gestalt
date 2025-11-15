#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyExplicitProxy {
    /// Whether the pac file port and url need to be provided.
    #[builder(into)]
    #[serde(rename = "enablePacFile")]
    pub r#enable_pac_file: Option<bool>,
    /// Whether the explicit proxy is enabled for this Firewall Policy.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The port number for explicit http protocol.
    #[builder(into)]
    #[serde(rename = "httpPort")]
    pub r#http_port: Option<i32>,
    /// The port number for explicit proxy https protocol.
    #[builder(into)]
    #[serde(rename = "httpsPort")]
    pub r#https_port: Option<i32>,
    /// Specifies a SAS URL for PAC file.
    #[builder(into)]
    #[serde(rename = "pacFile")]
    pub r#pac_file: Option<String>,
    /// Specifies a port number for firewall to serve PAC file.
    #[builder(into)]
    #[serde(rename = "pacFilePort")]
    pub r#pac_file_port: Option<i32>,
}
