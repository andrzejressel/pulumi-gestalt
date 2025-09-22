#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackendProxy {
    /// The password to connect to the proxy server.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The URL of the proxy server.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
    /// The username to connect to the proxy server.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}
