#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CatalogDatabaseFederatedDatabase {
    /// Name of the connection to the external metastore.
    #[builder(into, default)]
    #[serde(rename = "connectionName")]
    pub r#connection_name: Box<Option<String>>,
    /// Unique identifier for the federated database.
    #[builder(into, default)]
    #[serde(rename = "identifier")]
    pub r#identifier: Box<Option<String>>,
}
