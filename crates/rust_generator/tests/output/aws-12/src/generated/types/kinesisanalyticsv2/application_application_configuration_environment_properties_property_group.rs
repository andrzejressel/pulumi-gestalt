#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationEnvironmentPropertiesPropertyGroup {
    /// The key of the application execution property key-value map.
    #[builder(into)]
    #[serde(rename = "propertyGroupId")]
    pub r#property_group_id: String,
    /// Application execution property key-value map.
    #[builder(into)]
    #[serde(rename = "propertyMap")]
    pub r#property_map: std::collections::HashMap<String, String>,
}
