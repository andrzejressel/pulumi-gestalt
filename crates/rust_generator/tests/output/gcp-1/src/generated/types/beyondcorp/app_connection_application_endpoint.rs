#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppConnectionApplicationEndpoint {
    /// Hostname or IP address of the remote application endpoint.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: String,
    /// Port of the remote application endpoint.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
}
