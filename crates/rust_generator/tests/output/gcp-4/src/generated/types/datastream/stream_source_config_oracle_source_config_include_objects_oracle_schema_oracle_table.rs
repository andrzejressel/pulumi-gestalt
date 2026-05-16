#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamSourceConfigOracleSourceConfigIncludeObjectsOracleSchemaOracleTable {
    /// Oracle columns in the schema. When unspecified as part of include/exclude objects, includes/excludes everything.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oracleColumns")]
    pub r#oracle_columns: Option<Vec<super::super::types::datastream::StreamSourceConfigOracleSourceConfigIncludeObjectsOracleSchemaOracleTableOracleColumn>>,
    /// Table name.
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: String,
}
