#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSetColumnGroupGeoSpatialColumnGroup {
    /// Columns in this hierarchy.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Vec<String>,
    /// Country code. Valid values are `US`.
    #[builder(into)]
    #[serde(rename = "countryCode")]
    pub r#country_code: String,
    /// A display name for the hierarchy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
