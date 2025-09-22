#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCatalogTableStorageDescriptorSchemaReferenceSchemaId {
    /// Name of the schema registry that contains the schema.
    #[builder(into)]
    #[serde(rename = "registryName")]
    pub r#registry_name: String,
    /// ARN of the schema.
    #[builder(into)]
    #[serde(rename = "schemaArn")]
    pub r#schema_arn: String,
    /// Name of the schema.
    #[builder(into)]
    #[serde(rename = "schemaName")]
    pub r#schema_name: String,
}
