#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureGroupOfflineStoreConfigDataCatalogConfig {
    /// The name of the Glue table catalog.
    #[builder(into)]
    #[serde(rename = "catalog")]
    pub r#catalog: Option<String>,
    /// The name of the Glue table database.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: Option<String>,
    /// The name of the Glue table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: Option<String>,
}
