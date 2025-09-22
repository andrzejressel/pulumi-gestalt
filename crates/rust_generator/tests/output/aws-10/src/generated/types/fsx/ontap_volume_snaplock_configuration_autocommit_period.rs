#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OntapVolumeSnaplockConfigurationAutocommitPeriod {
    /// The type of time for the autocommit period of a file in an FSx for ONTAP SnapLock volume. Setting this value to `NONE` disables autocommit. Valid values: `MINUTES`, `HOURS`, `DAYS`, `MONTHS`, `YEARS`, `NONE`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// The amount of time for the autocommit period of a file in an FSx for ONTAP SnapLock volume.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<i32>,
}
