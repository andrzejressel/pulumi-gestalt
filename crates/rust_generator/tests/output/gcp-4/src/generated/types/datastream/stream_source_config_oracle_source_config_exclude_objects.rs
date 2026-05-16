#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamSourceConfigOracleSourceConfigExcludeObjects {
    /// Oracle schemas/databases in the database server
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oracleSchemas")]
    pub r#oracle_schemas: Vec<super::super::types::datastream::StreamSourceConfigOracleSourceConfigExcludeObjectsOracleSchema>,
}
