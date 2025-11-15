#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorBackendPoolBackend {
    /// Location of the backend (IP address or FQDN)
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: String,
    /// Specifies if the backend is enabled or not. Valid options are `true` or `false`. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The value to use as the host header sent to the backend.
    #[builder(into)]
    #[serde(rename = "hostHeader")]
    pub r#host_header: String,
    /// The HTTP TCP port number. Possible values are between `1` - `65535`.
    #[builder(into)]
    #[serde(rename = "httpPort")]
    pub r#http_port: i32,
    /// The HTTPS TCP port number. Possible values are between `1` - `65535`.
    #[builder(into)]
    #[serde(rename = "httpsPort")]
    pub r#https_port: i32,
    /// Priority to use for load balancing. Higher priorities will not be used for load balancing if any lower priority backend is healthy. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    /// Weight of this endpoint for load balancing purposes. Defaults to `50`.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Option<i32>,
}
