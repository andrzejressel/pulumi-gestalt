#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserPoolSchemaNumberAttributeConstraints {
    /// Maximum value of an attribute that is of the number data type.
    #[builder(into)]
    #[serde(rename = "maxValue")]
    pub r#max_value: Option<String>,
    /// Minimum value of an attribute that is of the number data type.
    #[builder(into)]
    #[serde(rename = "minValue")]
    pub r#min_value: Option<String>,
}
