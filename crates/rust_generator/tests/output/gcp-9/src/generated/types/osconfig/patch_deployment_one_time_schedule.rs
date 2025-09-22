#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PatchDeploymentOneTimeSchedule {
    /// The desired patch job execution time. A timestamp in RFC3339 UTC "Zulu" format,
    /// accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "executeTime")]
    pub r#execute_time: String,
}
