#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableTableConstraints {
    /// Present only if the table has a foreign key.
    /// The foreign key is not enforced.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "foreignKeys")]
    pub r#foreign_keys: Option<Vec<super::super::types::bigquery::TableTableConstraintsForeignKey>>,
    /// Represents the primary key constraint
    /// on a table's columns. Present only if the table has a primary key.
    /// The primary key is not enforced.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: Box<Option<super::super::types::bigquery::TableTableConstraintsPrimaryKey>>,
}
