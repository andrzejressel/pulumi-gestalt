#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSetPhysicalTableMapRelationalTable {
    #[builder(into)]
    #[serde(rename = "catalog")]
    pub r#catalog: String,
    #[builder(into)]
    #[serde(rename = "dataSourceArn")]
    pub r#data_source_arn: String,
    #[builder(into)]
    #[serde(rename = "inputColumns")]
    pub r#input_columns: Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapRelationalTableInputColumn>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: String,
}
