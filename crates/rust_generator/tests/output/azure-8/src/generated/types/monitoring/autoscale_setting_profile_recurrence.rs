#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutoscaleSettingProfileRecurrence {
    /// A list of days that this profile takes effect on. Possible values include `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: Vec<String>,
    /// A list containing a single item, which specifies the Hour interval at which this recurrence should be triggered (in 24-hour time). Possible values are from `0` to `23`.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: i32,
    /// A list containing a single item which specifies the Minute interval at which this recurrence should be triggered.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: i32,
    /// The Time Zone used for the `hours` field. A list of possible values can be found here). Defaults to `UTC`.
    #[builder(into)]
    #[serde(rename = "timezone")]
    pub r#timezone: Option<String>,
}
