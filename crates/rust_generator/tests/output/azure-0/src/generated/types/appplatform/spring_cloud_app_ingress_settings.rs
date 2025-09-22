#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudAppIngressSettings {
    /// Specifies how ingress should communicate with this app backend service. Allowed values are `GRPC` and `Default`. Defaults to `Default`.
    #[builder(into)]
    #[serde(rename = "backendProtocol")]
    pub r#backend_protocol: Option<String>,
    /// Specifies the ingress read time out in seconds. Defaults to `300`.
    #[builder(into)]
    #[serde(rename = "readTimeoutInSeconds")]
    pub r#read_timeout_in_seconds: Option<i32>,
    /// Specifies the ingress send time out in seconds. Defaults to `60`.
    #[builder(into)]
    #[serde(rename = "sendTimeoutInSeconds")]
    pub r#send_timeout_in_seconds: Option<i32>,
    /// Specifies the type of the affinity, set this to `Cookie` to enable session affinity. Allowed values are `Cookie` and `None`. Defaults to `None`.
    #[builder(into)]
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Option<String>,
    /// Specifies the time in seconds until the cookie expires.
    #[builder(into)]
    #[serde(rename = "sessionCookieMaxAge")]
    pub r#session_cookie_max_age: Option<i32>,
}
