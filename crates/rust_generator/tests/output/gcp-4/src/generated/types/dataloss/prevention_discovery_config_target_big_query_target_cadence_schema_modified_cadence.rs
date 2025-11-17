#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetCadenceSchemaModifiedCadence {
    /// Frequency to regenerate data profiles when the schema is modified. Defaults to monthly.
    /// Possible values are: `UPDATE_FREQUENCY_NEVER`, `UPDATE_FREQUENCY_DAILY`, `UPDATE_FREQUENCY_MONTHLY`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: Option<String>,
    /// The types of schema modifications to consider. Defaults to NEW_COLUMNS.
    /// Each value may be one of: `NEW_COLUMNS`, `REMOVED_COLUMNS`.
    #[builder(into)]
    #[serde(rename = "types")]
    pub r#types: Option<Vec<String>>,
}
