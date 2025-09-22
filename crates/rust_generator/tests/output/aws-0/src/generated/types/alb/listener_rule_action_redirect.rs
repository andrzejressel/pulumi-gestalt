#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerRuleActionRedirect {
    /// The hostname. This component is not percent-encoded. The hostname can contain `#{host}`. Defaults to `#{host}`.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// The absolute path, starting with the leading "/". This component is not percent-encoded. The path can contain #{host}, #{path}, and #{port}. Defaults to `/#{path}`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// The port. Specify a value from `1` to `65535` or `#{port}`. Defaults to `#{port}`.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<String>,
    /// The protocol. Valid values are `HTTP`, `HTTPS`, or `#{protocol}`. Defaults to `#{protocol}`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    /// The query parameters, URL-encoded when necessary, but not percent-encoded. Do not include the leading "?". Defaults to `#{query}`.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: Option<String>,
    /// The HTTP redirect code. The redirect is either permanent (`HTTP_301`) or temporary (`HTTP_302`).
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: String,
}
