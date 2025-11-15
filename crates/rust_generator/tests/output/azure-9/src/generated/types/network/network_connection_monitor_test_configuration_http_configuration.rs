#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkConnectionMonitorTestConfigurationHttpConfiguration {
    /// The HTTP method for the HTTP request. Possible values are `Get` and `Post`. Defaults to `Get`.
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: Option<String>,
    /// The path component of the URI. It only accepts the absolute path.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// The port for the HTTP connection.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Should HTTPS be preferred over HTTP in cases where the choice is not explicit? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "preferHttps")]
    pub r#prefer_https: Option<bool>,
    /// A `request_header` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestHeaders")]
    pub r#request_headers: Option<Vec<super::super::types::network::NetworkConnectionMonitorTestConfigurationHttpConfigurationRequestHeader>>,
    /// The HTTP status codes to consider successful. For instance, `2xx`, `301-304` and `418`.
    #[builder(into)]
    #[serde(rename = "validStatusCodeRanges")]
    pub r#valid_status_code_ranges: Option<Vec<String>>,
}
