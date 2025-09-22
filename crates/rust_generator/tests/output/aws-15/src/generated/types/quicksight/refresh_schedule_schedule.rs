#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RefreshScheduleSchedule {
    /// The type of refresh that the dataset undergoes. Valid values are `INCREMENTAL_REFRESH` and `FULL_REFRESH`.
    #[builder(into)]
    #[serde(rename = "refreshType")]
    pub r#refresh_type: String,
    /// The configuration of the [schedule frequency](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_RefreshFrequency.html). See schedule_frequency.
    #[builder(into)]
    #[serde(rename = "scheduleFrequency")]
    pub r#schedule_frequency: Option<Box<super::super::types::quicksight::RefreshScheduleScheduleScheduleFrequency>>,
    /// Time after which the refresh schedule can be started, expressed in `YYYY-MM-DDTHH:MM:SS` format.
    #[builder(into)]
    #[serde(rename = "startAfterDateTime")]
    pub r#start_after_date_time: Option<String>,
}
