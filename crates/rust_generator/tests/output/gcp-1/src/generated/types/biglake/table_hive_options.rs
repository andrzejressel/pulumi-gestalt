#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableHiveOptions {
    /// Stores user supplied Hive table parameters. An object containing a
    /// list of "key": value pairs.
    /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<std::collections::HashMap<String, String>>,
    /// Stores physical storage information on the data.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "storageDescriptor")]
    pub r#storage_descriptor: Option<Box<super::super::types::biglake::TableHiveOptionsStorageDescriptor>>,
    /// Hive table type. For example, MANAGED_TABLE, EXTERNAL_TABLE.
    #[builder(into)]
    #[serde(rename = "tableType")]
    pub r#table_type: Option<String>,
}
