#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointGlobalDeliveryRuleCacheKeyQueryStringAction {
    /// The behavior of the cache key for query strings. Valid values are `Exclude`, `ExcludeAll`, `Include` and `IncludeAll`.
    #[builder(into)]
    #[serde(rename = "behavior")]
    pub r#behavior: String,
    /// Comma separated list of parameter values.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<String>,
}
