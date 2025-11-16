#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableTableConstraintsForeignKey {
    /// The pair of the foreign key column and primary key column.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "columnReferences")]
    pub r#column_references: Box<super::super::types::bigquery::TableTableConstraintsForeignKeyColumnReferences>,
    /// Set only if the foreign key constraint is named.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The table that holds the primary key
    /// and is referenced by this foreign key.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "referencedTable")]
    pub r#referenced_table: Box<super::super::types::bigquery::TableTableConstraintsForeignKeyReferencedTable>,
}
