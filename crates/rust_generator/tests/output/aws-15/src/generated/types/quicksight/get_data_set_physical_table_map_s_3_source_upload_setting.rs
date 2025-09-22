#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSetPhysicalTableMapS3SourceUploadSetting {
    #[builder(into)]
    #[serde(rename = "containsHeader")]
    pub r#contains_header: bool,
    #[builder(into)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: String,
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: String,
    #[builder(into)]
    #[serde(rename = "startFromRow")]
    pub r#start_from_row: i32,
    #[builder(into)]
    #[serde(rename = "textQualifier")]
    pub r#text_qualifier: String,
}
