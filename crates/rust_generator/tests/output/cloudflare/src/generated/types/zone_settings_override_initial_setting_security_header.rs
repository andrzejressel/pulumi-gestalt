#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZoneSettingsOverrideInitialSettingSecurityHeader {
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Option<bool>,
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Option<i32>,
    #[builder(into)]
    #[serde(rename = "nosniff")]
    pub r#nosniff: Option<bool>,
    #[builder(into)]
    #[serde(rename = "preload")]
    pub r#preload: Option<bool>,
}
