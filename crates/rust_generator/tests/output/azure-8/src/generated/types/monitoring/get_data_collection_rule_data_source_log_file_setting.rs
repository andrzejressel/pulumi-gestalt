#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataCollectionRuleDataSourceLogFileSetting {
    /// A `text` block as defined below.
    #[builder(into)]
    #[serde(rename = "texts")]
    pub r#texts: Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceLogFileSettingText>,
}
