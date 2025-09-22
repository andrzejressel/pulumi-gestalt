#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionUrlMapPathMatcherRouteRuleMatchRuleMetadataFilterFilterLabel {
    /// Name of metadata label. The name can have a maximum length of 1024 characters
    /// and must be at least 1 character long.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The value of the label must match the specified value. value can have a maximum
    /// length of 1024 characters.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
