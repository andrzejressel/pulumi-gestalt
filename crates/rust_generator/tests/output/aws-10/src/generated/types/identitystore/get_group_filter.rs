#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupFilter {
    /// Attribute path that is used to specify which attribute name to search. Currently, `DisplayName` is the only valid attribute path.
    #[builder(into)]
    #[serde(rename = "attributePath")]
    pub r#attribute_path: String,
    /// Value for an attribute.
    #[builder(into)]
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: String,
}
