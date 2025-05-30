#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionCacheBehaviorSettingsForwardedHeaders {
    /// The specific headers to forward to your distribution's origin.
    #[builder(into, default)]
    #[serde(rename = "headersAllowLists")]
    pub r#headers_allow_lists: Box<Option<Vec<String>>>,
    /// The headers that you want your distribution to forward to your origin and base caching on.
    #[builder(into, default)]
    #[serde(rename = "option")]
    pub r#option: Box<Option<String>>,
}
