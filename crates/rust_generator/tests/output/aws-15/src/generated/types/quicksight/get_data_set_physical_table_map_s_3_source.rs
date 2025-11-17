#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataSetPhysicalTableMapS3Source {
    #[builder(into)]
    #[serde(rename = "dataSourceArn")]
    pub r#data_source_arn: String,
    #[builder(into)]
    #[serde(rename = "inputColumns")]
    pub r#input_columns: Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapS3SourceInputColumn>,
    #[builder(into)]
    #[serde(rename = "uploadSettings")]
    pub r#upload_settings: Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapS3SourceUploadSetting>,
}
