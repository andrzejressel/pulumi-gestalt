#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableMaintenanceConfigurationIcebergCompactionSettings {
    /// Data objects smaller than this size may be combined with others to improve query performance.
    /// Must be between `64` and `512`.
    #[builder(into)]
    #[serde(rename = "targetFileSizeMb")]
    pub r#target_file_size_mb: f64,
}
