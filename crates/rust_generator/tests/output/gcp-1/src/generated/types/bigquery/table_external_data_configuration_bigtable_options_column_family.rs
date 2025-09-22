#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableExternalDataConfigurationBigtableOptionsColumnFamily {
    /// A List of columns that should be exposed as individual fields as opposed to a list of (column name, value) pairs. All columns whose qualifier matches a qualifier in this list can be accessed as Other columns can be accessed as a list through column field.  Structure is documented below.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Option<Vec<super::super::types::bigquery::TableExternalDataConfigurationBigtableOptionsColumnFamilyColumn>>,
    /// The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. This can be overridden for a specific column by listing that column in 'columns' and specifying an encoding for it.
    #[builder(into)]
    #[serde(rename = "encoding")]
    pub r#encoding: Option<String>,
    /// Identifier of the column family.
    #[builder(into)]
    #[serde(rename = "familyId")]
    pub r#family_id: Option<String>,
    /// If this is set only the latest version of value are exposed for all columns in this column family. This can be overridden for a specific column by listing that column in 'columns' and specifying a different setting for that column.
    #[builder(into)]
    #[serde(rename = "onlyReadLatest")]
    pub r#only_read_latest: Option<bool>,
    /// The type to convert the value in cells of this column family. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive): "BYTES", "STRING", "INTEGER", "FLOAT", "BOOLEAN", "JSON". Default type is BYTES. This can be overridden for a specific column by listing that column in 'columns' and specifying a type for it.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
