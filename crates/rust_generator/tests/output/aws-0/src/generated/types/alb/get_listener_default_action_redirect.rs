#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetListenerDefaultActionRedirect {
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// Port of the listener. Required if `arn` is not set.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<String>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: Box<String>,
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<String>,
}
