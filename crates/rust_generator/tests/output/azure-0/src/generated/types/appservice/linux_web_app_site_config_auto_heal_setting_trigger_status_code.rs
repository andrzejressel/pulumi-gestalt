#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxWebAppSiteConfigAutoHealSettingTriggerStatusCode {
    /// The number of occurrences of the defined `status_code` in the specified `interval` on which to trigger this rule.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// The time interval in the form `hh:mm:ss`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: String,
    /// The path to which this rule status code applies.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// The status code for this rule, accepts single status codes and status code ranges. e.g. `500` or `400-499`. Possible values are integers between `101` and `599`
    #[builder(into)]
    #[serde(rename = "statusCodeRange")]
    pub r#status_code_range: String,
    /// The Request Sub Status of the Status Code.
    #[builder(into)]
    #[serde(rename = "subStatus")]
    pub r#sub_status: Option<i32>,
    /// The Win32 Status Code of the Request.
    #[builder(into)]
    #[serde(rename = "win32StatusCode")]
    pub r#win_32_status_code: Option<i32>,
}
