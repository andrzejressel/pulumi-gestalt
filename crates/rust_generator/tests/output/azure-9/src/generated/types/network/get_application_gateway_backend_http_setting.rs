#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewayBackendHttpSetting {
    /// The name of the affinity cookie.
    #[builder(into)]
    #[serde(rename = "affinityCookieName")]
    pub r#affinity_cookie_name: String,
    /// One or more `authentication_certificate` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "authenticationCertificates")]
    pub r#authentication_certificates: Vec<super::super::types::network::GetApplicationGatewayBackendHttpSettingAuthenticationCertificate>,
    /// A `connection_draining` block as defined below.
    #[builder(into)]
    #[serde(rename = "connectionDrainings")]
    pub r#connection_drainings: Vec<super::super::types::network::GetApplicationGatewayBackendHttpSettingConnectionDraining>,
    /// Is Cookie-Based Affinity enabled?
    #[builder(into)]
    #[serde(rename = "cookieBasedAffinity")]
    pub r#cookie_based_affinity: String,
    /// The Hostname which is used for this HTTP Listener.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: String,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The URL path to rewrite.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// Whether host header will be picked from the host name of the backend server.
    #[builder(into)]
    #[serde(rename = "pickHostNameFromBackendAddress")]
    pub r#pick_host_name_from_backend_address: bool,
    /// Custom port which is used for probing the backend servers.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// The ID of the associated Probe.
    #[builder(into)]
    #[serde(rename = "probeId")]
    pub r#probe_id: String,
    /// The name of the associated HTTP Probe.
    #[builder(into)]
    #[serde(rename = "probeName")]
    pub r#probe_name: String,
    /// The Protocol used for this Probe.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// The request timeout in seconds.
    #[builder(into)]
    #[serde(rename = "requestTimeout")]
    pub r#request_timeout: i32,
    /// A list of `trusted_root_certificate` names.
    #[builder(into)]
    #[serde(rename = "trustedRootCertificateNames")]
    pub r#trusted_root_certificate_names: Vec<String>,
}
