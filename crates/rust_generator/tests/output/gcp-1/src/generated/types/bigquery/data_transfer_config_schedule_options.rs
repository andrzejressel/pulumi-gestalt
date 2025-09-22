#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataTransferConfigScheduleOptions {
    /// If true, automatic scheduling of data transfer runs for this
    /// configuration will be disabled. The runs can be started on ad-hoc
    /// basis using transferConfigs.startManualRuns API. When automatic
    /// scheduling is disabled, the TransferConfig.schedule field will
    /// be ignored.
    #[builder(into)]
    #[serde(rename = "disableAutoScheduling")]
    pub r#disable_auto_scheduling: Option<bool>,
    /// Defines time to stop scheduling transfer runs. A transfer run cannot be
    /// scheduled at or after the end time. The end time can be changed at any
    /// moment. The time when a data transfer can be triggered manually is not
    /// limited by this option.
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Option<String>,
    /// Specifies time to start scheduling transfer runs. The first run will be
    /// scheduled at or after the start time according to a recurrence pattern
    /// defined in the schedule string. The start time can be changed at any
    /// moment. The time when a data transfer can be triggered manually is not
    /// limited by this option.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
}
