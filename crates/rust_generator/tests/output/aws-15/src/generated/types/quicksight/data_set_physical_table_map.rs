#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSetPhysicalTableMap {
    /// A physical table type built from the results of the custom SQL query. See custom_sql.
    #[builder(into)]
    #[serde(rename = "customSql")]
    pub r#custom_sql: Option<Box<super::super::types::quicksight::DataSetPhysicalTableMapCustomSql>>,
    /// Key of the physical table map.
    #[builder(into)]
    #[serde(rename = "physicalTableMapId")]
    pub r#physical_table_map_id: String,
    /// A physical table type for relational data sources. See relational_table.
    #[builder(into)]
    #[serde(rename = "relationalTable")]
    pub r#relational_table: Option<Box<super::super::types::quicksight::DataSetPhysicalTableMapRelationalTable>>,
    /// A physical table type for as S3 data source. See s3_source.
    #[builder(into)]
    #[serde(rename = "s3Source")]
    pub r#s_3_source: Option<Box<super::super::types::quicksight::DataSetPhysicalTableMapS3Source>>,
}
