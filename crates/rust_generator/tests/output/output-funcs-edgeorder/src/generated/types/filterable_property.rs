#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FilterableProperty {
    /// Values to be filtered.
    #[builder(into)]
    #[serde(rename = "supportedValues")]
    pub r#supported_values: Vec<String>,
    /// Type of product filter.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: pulumi_gestalt_rust::OneOf2<String, Box<super::types::SupportedFilterTypes>>,
}
