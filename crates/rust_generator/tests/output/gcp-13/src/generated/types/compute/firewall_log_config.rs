#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallLogConfig {
    /// This field denotes whether to include or exclude metadata for firewall logs.
    /// Possible values are: `EXCLUDE_ALL_METADATA`, `INCLUDE_ALL_METADATA`.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: String,
}
