#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterNodePoolNodeConfigEffectiveTaint {
    /// Effect for taint.
    #[builder(into)]
    #[serde(rename = "effect")]
    pub r#effect: String,
    /// Key for taint.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Value for taint.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
