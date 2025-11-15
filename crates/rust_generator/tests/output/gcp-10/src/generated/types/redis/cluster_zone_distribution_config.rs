#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterZoneDistributionConfig {
    /// Immutable. The mode for zone distribution for Memorystore Redis cluster.
    /// If not provided, MULTI_ZONE will be used as default
    /// Possible values are: `MULTI_ZONE`, `SINGLE_ZONE`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// Immutable. The zone for single zone Memorystore Redis cluster.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Option<String>,
}
