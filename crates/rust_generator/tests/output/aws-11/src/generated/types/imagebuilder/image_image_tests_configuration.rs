#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ImageImageTestsConfiguration {
    /// Whether image tests are enabled. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "imageTestsEnabled")]
    pub r#image_tests_enabled: Option<bool>,
    /// Number of minutes before image tests time out. Valid values are between `60` and `1440`. Defaults to `720`.
    #[builder(into)]
    #[serde(rename = "timeoutMinutes")]
    pub r#timeout_minutes: Option<i32>,
}
