#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodePoolManagement {
    /// Whether the nodes will be automatically repaired. Enabled by default.
    #[builder(into)]
    #[serde(rename = "autoRepair")]
    pub r#auto_repair: bool,
    /// Whether the nodes will be automatically upgraded. Enabled by default.
    #[builder(into)]
    #[serde(rename = "autoUpgrade")]
    pub r#auto_upgrade: bool,
}
