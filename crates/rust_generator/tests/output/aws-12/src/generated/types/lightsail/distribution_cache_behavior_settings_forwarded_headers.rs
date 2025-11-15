#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionCacheBehaviorSettingsForwardedHeaders {
    /// The specific headers to forward to your distribution's origin.
    #[builder(into)]
    #[serde(rename = "headersAllowLists")]
    pub r#headers_allow_lists: Option<Vec<String>>,
    /// The headers that you want your distribution to forward to your origin and base caching on.
    #[builder(into)]
    #[serde(rename = "option")]
    pub r#option: Option<String>,
}
