#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RunCommandInstanceView {
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Option<String>,
    #[builder(into)]
    #[serde(rename = "errorMessage")]
    pub r#error_message: Option<String>,
    #[builder(into)]
    #[serde(rename = "executionMessage")]
    pub r#execution_message: Option<String>,
    #[builder(into)]
    #[serde(rename = "executionState")]
    pub r#execution_state: Option<String>,
    #[builder(into)]
    #[serde(rename = "exitCode")]
    pub r#exit_code: Option<i32>,
    #[builder(into)]
    #[serde(rename = "output")]
    pub r#output: Option<String>,
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
}
