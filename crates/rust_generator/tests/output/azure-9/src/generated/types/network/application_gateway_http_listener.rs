#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewayHttpListener {
    /// One or more `custom_error_configuration` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "customErrorConfigurations")]
    pub r#custom_error_configurations: Option<Vec<super::super::types::network::ApplicationGatewayHttpListenerCustomErrorConfiguration>>,
    /// The ID of the Web Application Firewall Policy which should be used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "firewallPolicyId")]
    pub r#firewall_policy_id: Option<String>,
    /// The ID of the associated Frontend Configuration.
    #[builder(into)]
    #[serde(rename = "frontendIpConfigurationId")]
    pub r#frontend_ip_configuration_id: Option<String>,
    /// The Name of the Frontend IP Configuration used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "frontendIpConfigurationName")]
    pub r#frontend_ip_configuration_name: String,
    /// The ID of the associated Frontend Port.
    #[builder(into)]
    #[serde(rename = "frontendPortId")]
    pub r#frontend_port_id: Option<String>,
    /// The Name of the Frontend Port use for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "frontendPortName")]
    pub r#frontend_port_name: String,
    /// The Hostname which should be used for this HTTP Listener. Setting this value changes Listener Type to 'Multi site'.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Option<String>,
    /// A list of Hostname(s) should be used for this HTTP Listener. It allows special wildcard characters.
    /// 
    /// > **NOTE** The `host_names` and `host_name` are mutually exclusive and cannot both be set.
    #[builder(into)]
    #[serde(rename = "hostNames")]
    pub r#host_names: Option<Vec<String>>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The Name of the HTTP Listener.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The Protocol to use for this HTTP Listener. Possible values are `Http` and `Https`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// Should Server Name Indication be Required? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "requireSni")]
    pub r#require_sni: Option<bool>,
    /// The ID of the associated SSL Certificate.
    #[builder(into)]
    #[serde(rename = "sslCertificateId")]
    pub r#ssl_certificate_id: Option<String>,
    /// The name of the associated SSL Certificate which should be used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "sslCertificateName")]
    pub r#ssl_certificate_name: Option<String>,
    /// The ID of the associated SSL Profile.
    #[builder(into)]
    #[serde(rename = "sslProfileId")]
    pub r#ssl_profile_id: Option<String>,
    /// The name of the associated SSL Profile which should be used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "sslProfileName")]
    pub r#ssl_profile_name: Option<String>,
}
