#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTargetCloudStorageTargetConditions {
    /// Cloud Storage conditions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudStorageConditions")]
    pub r#cloud_storage_conditions: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetConditionsCloudStorageConditions>>,
    /// File store must have been created after this date. Used to avoid backfilling. A timestamp in RFC3339 UTC "Zulu" format with nanosecond resolution and upto nine fractional digits.
    #[builder(into)]
    #[serde(rename = "createdAfter")]
    pub r#created_after: Option<String>,
    /// Duration format. Minimum age a file store must have. If set, the value must be 1 hour or greater.
    #[builder(into)]
    #[serde(rename = "minAge")]
    pub r#min_age: Option<String>,
}
