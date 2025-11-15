#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupContainerReadinessProbeHttpGet {
    /// A map of HTTP headers used to access on the container. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "httpHeaders")]
    pub r#http_headers: Option<std::collections::HashMap<String, String>>,
    /// Path to access on the HTTP server. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Number of the port to access on the container. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Scheme to use for connecting to the host. Possible values are `Http` and `Https`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "scheme")]
    pub r#scheme: Option<String>,
}
