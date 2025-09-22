#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationGatewayBackendHttpSetting {
    /// The name of the affinity cookie.
    #[builder(into)]
    #[serde(rename = "affinityCookieName")]
    pub r#affinity_cookie_name: Option<String>,
    /// One or more `authentication_certificate_backend` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "authenticationCertificates")]
    pub r#authentication_certificates: Option<Vec<super::super::types::network::ApplicationGatewayBackendHttpSettingAuthenticationCertificate>>,
    /// A `connection_draining` block as defined below.
    #[builder(into)]
    #[serde(rename = "connectionDraining")]
    pub r#connection_draining: Option<Box<super::super::types::network::ApplicationGatewayBackendHttpSettingConnectionDraining>>,
    /// Is Cookie-Based Affinity enabled? Possible values are `Enabled` and `Disabled`.
    #[builder(into)]
    #[serde(rename = "cookieBasedAffinity")]
    pub r#cookie_based_affinity: String,
    /// Host header to be sent to the backend servers. Cannot be set if `pick_host_name_from_backend_address` is set to `true`.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Option<String>,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The name of the Authentication Certificate.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The Path which should be used as a prefix for all HTTP requests.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Whether host header should be picked from the host name of the backend server. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "pickHostNameFromBackendAddress")]
    pub r#pick_host_name_from_backend_address: Option<bool>,
    /// The port which should be used for this Backend HTTP Settings Collection.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// The ID of the associated Probe.
    #[builder(into)]
    #[serde(rename = "probeId")]
    pub r#probe_id: Option<String>,
    /// The name of an associated HTTP Probe.
    #[builder(into)]
    #[serde(rename = "probeName")]
    pub r#probe_name: Option<String>,
    /// The Protocol which should be used. Possible values are `Http` and `Https`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// The request timeout in seconds, which must be between 1 and 86400 seconds. Defaults to `30`.
    #[builder(into)]
    #[serde(rename = "requestTimeout")]
    pub r#request_timeout: Option<i32>,
    /// A list of `trusted_root_certificate` names.
    #[builder(into)]
    #[serde(rename = "trustedRootCertificateNames")]
    pub r#trusted_root_certificate_names: Option<Vec<String>>,
}
