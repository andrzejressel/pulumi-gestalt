#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamSourceConfigSqlServerSourceConfigIncludeObjectsSchemaTableColumn {
    /// Column name.
    #[builder(into)]
    #[serde(rename = "column")]
    pub r#column: Option<String>,
    /// The SQL Server data type. Full data types list can be found here:
    /// https://learn.microsoft.com/en-us/sql/t-sql/data-types/data-types-transact-sql?view=sql-server-ver16
    #[builder(into)]
    #[serde(rename = "dataType")]
    pub r#data_type: Option<String>,
    /// (Output)
    /// Column length.
    #[builder(into)]
    #[serde(rename = "length")]
    pub r#length: Option<i32>,
    /// (Output)
    /// Whether or not the column can accept a null value.
    #[builder(into)]
    #[serde(rename = "nullable")]
    pub r#nullable: Option<bool>,
    /// (Output)
    /// The ordinal position of the column in the table.
    #[builder(into)]
    #[serde(rename = "ordinalPosition")]
    pub r#ordinal_position: Option<i32>,
    /// (Output)
    /// Column precision.
    #[builder(into)]
    #[serde(rename = "precision")]
    pub r#precision: Option<i32>,
    /// (Output)
    /// Whether or not the column represents a primary key.
    #[builder(into)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: Option<bool>,
    /// (Output)
    /// Column scale.
    #[builder(into)]
    #[serde(rename = "scale")]
    pub r#scale: Option<i32>,
}
