#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterMaintenancePolicyMaintenanceExclusion {
    /// A unique (per cluster) id for the window.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Represents an arbitrary window of time.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "window")]
    pub r#window: Option<Box<super::super::types::edgecontainer::ClusterMaintenancePolicyMaintenanceExclusionWindow>>,
}
