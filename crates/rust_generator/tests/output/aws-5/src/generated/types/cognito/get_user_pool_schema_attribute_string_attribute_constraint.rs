#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserPoolSchemaAttributeStringAttributeConstraint {
    /// - Maximum allowed length.
    #[builder(into)]
    #[serde(rename = "maxLength")]
    pub r#max_length: String,
    /// - Minimum allowed length.
    #[builder(into)]
    #[serde(rename = "minLength")]
    pub r#min_length: String,
}
