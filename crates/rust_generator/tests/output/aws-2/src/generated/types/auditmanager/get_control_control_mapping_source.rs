#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetControlControlMappingSource {
    #[builder(into)]
    #[serde(rename = "sourceDescription")]
    pub r#source_description: String,
    #[builder(into)]
    #[serde(rename = "sourceFrequency")]
    pub r#source_frequency: String,
    #[builder(into)]
    #[serde(rename = "sourceId")]
    pub r#source_id: String,
    #[builder(into)]
    #[serde(rename = "sourceKeyword")]
    pub r#source_keyword: Box<Option<super::super::types::auditmanager::GetControlControlMappingSourceSourceKeyword>>,
    #[builder(into)]
    #[serde(rename = "sourceName")]
    pub r#source_name: String,
    #[builder(into)]
    #[serde(rename = "sourceSetUpOption")]
    pub r#source_set_up_option: String,
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: String,
    #[builder(into)]
    #[serde(rename = "troubleshootingText")]
    pub r#troubleshooting_text: String,
}
