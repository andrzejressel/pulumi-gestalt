#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetPhysicalTableMapRelationalTable {
    /// Catalog associated with the table.
    #[builder(into)]
    #[serde(rename = "catalog")]
    pub r#catalog: Option<String>,
    /// ARN of the data source.
    #[builder(into)]
    #[serde(rename = "dataSourceArn")]
    pub r#data_source_arn: String,
    /// Column schema of the table. See input_columns.
    #[builder(into)]
    #[serde(rename = "inputColumns")]
    pub r#input_columns: Vec<super::super::types::quicksight::DataSetPhysicalTableMapRelationalTableInputColumn>,
    /// Name of the relational table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Schema name. This name applies to certain relational database engines.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Option<String>,
}
