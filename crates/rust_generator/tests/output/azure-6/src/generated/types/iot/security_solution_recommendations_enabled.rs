#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecuritySolutionRecommendationsEnabled {
    /// Is Principal Authentication enabled for the ACR repository? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "acrAuthentication")]
    pub r#acr_authentication: Option<bool>,
    /// Is Agent send underutilized messages enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "agentSendUnutilizedMsg")]
    pub r#agent_send_unutilized_msg: Option<bool>,
    /// Is Security related system configuration issues identified? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "baseline")]
    pub r#baseline: Option<bool>,
    /// Is IoT Edge Hub memory optimized? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "edgeHubMemOptimize")]
    pub r#edge_hub_mem_optimize: Option<bool>,
    /// Is logging configured for IoT Edge module? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "edgeLoggingOption")]
    pub r#edge_logging_option: Option<bool>,
    /// Is inconsistent module settings enabled for SecurityGroup? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "inconsistentModuleSettings")]
    pub r#inconsistent_module_settings: Option<bool>,
    /// is Azure IoT Security agent installed? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "installAgent")]
    pub r#install_agent: Option<bool>,
    /// Is Default IP filter policy denied? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "ipFilterDenyAll")]
    pub r#ip_filter_deny_all: Option<bool>,
    /// Is IP filter rule source allowable IP range too large? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "ipFilterPermissiveRule")]
    pub r#ip_filter_permissive_rule: Option<bool>,
    /// Is any ports open on the device? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "openPorts")]
    pub r#open_ports: Option<bool>,
    /// Does firewall policy exist which allow necessary communication to/from the device? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "permissiveFirewallPolicy")]
    pub r#permissive_firewall_policy: Option<bool>,
    /// Is only necessary addresses or ports are permitted in? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "permissiveInputFirewallRules")]
    pub r#permissive_input_firewall_rules: Option<bool>,
    /// Is only necessary addresses or ports are permitted out? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "permissiveOutputFirewallRules")]
    pub r#permissive_output_firewall_rules: Option<bool>,
    /// Is high level permissions are needed for the module? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "privilegedDockerOptions")]
    pub r#privileged_docker_options: Option<bool>,
    /// Is any credentials shared among devices? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "sharedCredentials")]
    pub r#shared_credentials: Option<bool>,
    /// Does TLS cipher suite need to be updated? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "vulnerableTlsCipherSuite")]
    pub r#vulnerable_tls_cipher_suite: Option<bool>,
}
