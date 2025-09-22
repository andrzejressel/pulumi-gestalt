#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSetPhysicalTableMap {
    #[builder(into)]
    #[serde(rename = "customSqls")]
    pub r#custom_sqls: Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapCustomSql>,
    #[builder(into)]
    #[serde(rename = "physicalTableMapId")]
    pub r#physical_table_map_id: String,
    #[builder(into)]
    #[serde(rename = "relationalTables")]
    pub r#relational_tables: Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapRelationalTable>,
    #[builder(into)]
    #[serde(rename = "s3Sources")]
    pub r#s_3_sources: Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapS3Source>,
}
