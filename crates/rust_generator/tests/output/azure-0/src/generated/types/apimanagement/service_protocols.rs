#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceProtocols {
    /// Should HTTP/2 be supported by the API Management Service? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enableHttp2")]
    pub r#enable_http_2: Option<bool>,
}
