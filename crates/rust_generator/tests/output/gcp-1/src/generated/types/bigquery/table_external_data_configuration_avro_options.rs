#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableExternalDataConfigurationAvroOptions {
    /// If is set to true, indicates whether
    /// to interpret logical types as the corresponding BigQuery data type
    /// (for example, TIMESTAMP), instead of using the raw type (for example, INTEGER).
    #[builder(into)]
    #[serde(rename = "useAvroLogicalTypes")]
    pub r#use_avro_logical_types: bool,
}
