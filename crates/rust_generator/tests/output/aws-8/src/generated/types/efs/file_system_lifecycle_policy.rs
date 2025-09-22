#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FileSystemLifecyclePolicy {
    /// Indicates how long it takes to transition files to the archive storage class. Requires transition_to_ia, Elastic Throughput and General Purpose performance mode. Valid values: `AFTER_1_DAY`, `AFTER_7_DAYS`, `AFTER_14_DAYS`, `AFTER_30_DAYS`, `AFTER_60_DAYS`, `AFTER_90_DAYS`, `AFTER_180_DAYS`, `AFTER_270_DAYS`, or `AFTER_365_DAYS`.
    #[builder(into)]
    #[serde(rename = "transitionToArchive")]
    pub r#transition_to_archive: Option<String>,
    /// Indicates how long it takes to transition files to the IA storage class. Valid values: `AFTER_1_DAY`, `AFTER_7_DAYS`, `AFTER_14_DAYS`, `AFTER_30_DAYS`, `AFTER_60_DAYS`, `AFTER_90_DAYS`, `AFTER_180_DAYS`, `AFTER_270_DAYS`, or `AFTER_365_DAYS`.
    #[builder(into)]
    #[serde(rename = "transitionToIa")]
    pub r#transition_to_ia: Option<String>,
    /// Describes the policy used to transition a file from infequent access storage to primary storage. Valid values: `AFTER_1_ACCESS`.
    #[builder(into)]
    #[serde(rename = "transitionToPrimaryStorageClass")]
    pub r#transition_to_primary_storage_class: Option<String>,
}
