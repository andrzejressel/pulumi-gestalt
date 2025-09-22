#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LaunchConfigurationMetadataOptions {
    /// The state of the metadata service: `enabled`, `disabled`.
    #[builder(into)]
    #[serde(rename = "httpEndpoint")]
    pub r#http_endpoint: Option<String>,
    /// The desired HTTP PUT response hop limit for instance metadata requests.
    #[builder(into)]
    #[serde(rename = "httpPutResponseHopLimit")]
    pub r#http_put_response_hop_limit: Option<i32>,
    /// If session tokens are required: `optional`, `required`.
    #[builder(into)]
    #[serde(rename = "httpTokens")]
    pub r#http_tokens: Option<String>,
}
