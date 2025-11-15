#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeliveryRuleCacheExpirationAction {
    /// The behavior of the cache. Valid values are `BypassCache`, `Override` and `SetIfMissing`.
    #[builder(into)]
    #[serde(rename = "behavior")]
    pub r#behavior: String,
    /// Duration of the cache. Only allowed when `behavior` is set to `Override` or `SetIfMissing`. Format: `[d.]hh:mm:ss`
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Option<String>,
}
