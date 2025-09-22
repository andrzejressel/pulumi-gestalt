#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateConfigAdBreak {
    /// Start time in seconds for the ad break, relative to the output file timeline
    #[builder(into)]
    #[serde(rename = "startTimeOffset")]
    pub r#start_time_offset: Option<String>,
}
