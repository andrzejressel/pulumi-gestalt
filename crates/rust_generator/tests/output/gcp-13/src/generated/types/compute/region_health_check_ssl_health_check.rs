#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionHealthCheckSslHealthCheck {
    /// The TCP port number for the HTTP2 health check request.
    /// The default value is 443.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Port name as defined in InstanceGroup#NamedPort#name. If both port and
    /// port_name are defined, port takes precedence.
    #[builder(into)]
    #[serde(rename = "portName")]
    pub r#port_name: Option<String>,
    /// Specifies how port is selected for health checking, can be one of the
    /// following values:
    /// * `USE_FIXED_PORT`: The port number in `port` is used for health checking.
    /// * `USE_NAMED_PORT`: The `portName` is used for health checking.
    /// * `USE_SERVING_PORT`: For NetworkEndpointGroup, the port specified for each
    /// network endpoint is used for health checking. For other backends, the
    /// port or named port specified in the Backend Service is used for health
    /// checking.
    /// If not specified, HTTP2 health check follows behavior specified in `port` and
    /// `portName` fields.
    /// Possible values are: `USE_FIXED_PORT`, `USE_NAMED_PORT`, `USE_SERVING_PORT`.
    #[builder(into)]
    #[serde(rename = "portSpecification")]
    pub r#port_specification: Option<String>,
    /// Specifies the type of proxy header to append before sending data to the
    /// backend.
    /// Default value is `NONE`.
    /// Possible values are: `NONE`, `PROXY_V1`.
    #[builder(into)]
    #[serde(rename = "proxyHeader")]
    pub r#proxy_header: Option<String>,
    /// The application data to send once the SSL connection has been
    /// established (default value is empty). If both request and response are
    /// empty, the connection establishment alone will indicate health. The request
    /// data can only be ASCII.
    #[builder(into)]
    #[serde(rename = "request")]
    pub r#request: Option<String>,
    /// The bytes to match against the beginning of the response data. If left empty
    /// (the default value), any response will indicate health. The response data
    /// can only be ASCII.
    #[builder(into)]
    #[serde(rename = "response")]
    pub r#response: Option<String>,
}
