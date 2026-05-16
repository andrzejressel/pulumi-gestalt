#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableMaintenanceConfigurationIcebergSnapshotManagement {
    /// Settings for snapshot management.
    /// See `iceberg_snapshot_management.settings` below
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Box<super::super::types::s3tables::TableMaintenanceConfigurationIcebergSnapshotManagementSettings>,
    /// Whether the configuration is enabled.
    /// Valid values are `enabled` and `disabled`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
}
