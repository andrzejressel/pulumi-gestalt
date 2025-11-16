#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CatalogTableStorageDescriptorSchemaReferenceSchemaId {
    /// Name of the schema registry that contains the schema. Must be provided when `schema_name` is specified and conflicts with `schema_arn`.
    #[builder(into)]
    #[serde(rename = "registryName")]
    pub r#registry_name: Option<String>,
    /// ARN of the schema. One of `schema_arn` or `schema_name` has to be provided.
    #[builder(into)]
    #[serde(rename = "schemaArn")]
    pub r#schema_arn: Option<String>,
    /// Name of the schema. One of `schema_arn` or `schema_name` has to be provided.
    #[builder(into)]
    #[serde(rename = "schemaName")]
    pub r#schema_name: Option<String>,
}
