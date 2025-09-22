#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CatalogTableOptimizerConfigurationOrphanFileDeletionConfigurationIcebergConfiguration {
    /// Specifies a directory in which to look for files. You may choose a sub-directory rather than the top-level table location. Defaults to the table's location.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// The number of days that orphan files should be retained before file deletion. Defaults to `3`.
    #[builder(into)]
    #[serde(rename = "orphanFileRetentionPeriodInDays")]
    pub r#orphan_file_retention_period_in_days: Option<f64>,
}
