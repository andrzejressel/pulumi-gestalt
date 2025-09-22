#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct QueueHttpTargetUriOverride {
    /// Host override.
    /// When specified, replaces the host part of the task URL.
    /// For example, if the task URL is "https://www.google.com", and host value
    /// is set to "example.net", the overridden URI will be changed to "https://example.net".
    /// Host value cannot be an empty string (INVALID_ARGUMENT).
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// URI path.
    /// When specified, replaces the existing path of the task URL.
    /// Setting the path value to an empty string clears the URI path segment.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pathOverride")]
    pub r#path_override: Option<Box<super::super::types::cloudtasks::QueueHttpTargetUriOverridePathOverride>>,
    /// Port override.
    /// When specified, replaces the port part of the task URI.
    /// For instance, for a URI http://www.google.com/foo and port=123, the overridden URI becomes http://www.google.com:123/foo.
    /// Note that the port value must be a positive integer.
    /// Setting the port to 0 (Zero) clears the URI port.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<String>,
    /// URI query.
    /// When specified, replaces the query part of the task URI. Setting the query value to an empty string clears the URI query segment.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "queryOverride")]
    pub r#query_override: Option<Box<super::super::types::cloudtasks::QueueHttpTargetUriOverrideQueryOverride>>,
    /// Scheme override.
    /// When specified, the task URI scheme is replaced by the provided value (HTTP or HTTPS).
    /// Possible values are: `HTTP`, `HTTPS`.
    #[builder(into)]
    #[serde(rename = "scheme")]
    pub r#scheme: Option<String>,
    /// URI Override Enforce Mode
    /// When specified, determines the Target UriOverride mode. If not specified, it defaults to ALWAYS.
    /// Possible values are: `ALWAYS`, `IF_NOT_EXISTS`.
    #[builder(into)]
    #[serde(rename = "uriOverrideEnforceMode")]
    pub r#uri_override_enforce_mode: Option<String>,
}
