#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LifecyclePolicyPolicyDetailsScheduleFastRestoreRule {
    /// The Availability Zones in which to enable fast snapshot restore.
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Vec<String>,
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Option<i32>,
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "intervalUnit")]
    pub r#interval_unit: Option<String>,
}
