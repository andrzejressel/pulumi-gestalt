#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceZoneDistributionConfig {
    /// Optional. Current zone distribution mode. Defaults to MULTI_ZONE.
    /// Possible values:
    /// MULTI_ZONE
    /// SINGLE_ZONE
    /// Possible values are: `MULTI_ZONE`, `SINGLE_ZONE`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// Optional. Defines zone where all resources will be allocated with SINGLE_ZONE mode.
    /// Ignored for MULTI_ZONE mode.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Option<String>,
}
