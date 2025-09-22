#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TeamsAccountLoggingSettingsByRuleTypeDns {
    /// Whether to log all activity.
    #[builder(into)]
    #[serde(rename = "logAll")]
    pub r#log_all: bool,
    #[builder(into)]
    #[serde(rename = "logBlocks")]
    pub r#log_blocks: bool,
}
