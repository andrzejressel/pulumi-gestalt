#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InfrastructureConfigurationInstanceMetadataOptions {
    /// The number of hops that an instance can traverse to reach its destonation.
    #[builder(into)]
    #[serde(rename = "httpPutResponseHopLimit")]
    pub r#http_put_response_hop_limit: Option<i32>,
    /// Whether a signed token is required for instance metadata retrieval requests. Valid values: `required`, `optional`.
    #[builder(into)]
    #[serde(rename = "httpTokens")]
    pub r#http_tokens: Option<String>,
}
