#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelHlsIngestIngestEndpoint {
    /// The password
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The URL
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// The username
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
