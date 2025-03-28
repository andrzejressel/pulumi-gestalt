#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZoneSettingsOverrideSettingsSecurityHeader {
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "nosniff")]
    pub r#nosniff: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "preload")]
    pub r#preload: Box<Option<bool>>,
}
