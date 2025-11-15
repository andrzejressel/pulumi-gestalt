#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceObservabilityConfig {
    /// Observability feature status for an instance.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Query string length. The default value is 10240. Any integer between 1024 and 100000 is considered valid.
    #[builder(into)]
    #[serde(rename = "maxQueryStringLength")]
    pub r#max_query_string_length: Option<i32>,
    /// Preserve comments in the query string.
    #[builder(into)]
    #[serde(rename = "preserveComments")]
    pub r#preserve_comments: Option<bool>,
    /// Number of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 200 is considered valid.
    #[builder(into)]
    #[serde(rename = "queryPlansPerMinute")]
    pub r#query_plans_per_minute: Option<i32>,
    /// Record application tags for an instance. This flag is turned "on" by default.
    #[builder(into)]
    #[serde(rename = "recordApplicationTags")]
    pub r#record_application_tags: Option<bool>,
    /// Track actively running queries. If not set, default value is "off".
    #[builder(into)]
    #[serde(rename = "trackActiveQueries")]
    pub r#track_active_queries: Option<bool>,
    /// Record wait event types during query execution for an instance.
    #[builder(into)]
    #[serde(rename = "trackWaitEventTypes")]
    pub r#track_wait_event_types: Option<bool>,
    /// Record wait events during query execution for an instance.
    #[builder(into)]
    #[serde(rename = "trackWaitEvents")]
    pub r#track_wait_events: Option<bool>,
}
