#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalClusterValidationCheckStatusResult {
    /// (Output)
    /// The category of the validation.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: Option<String>,
    /// (Output)
    /// The description of the validation check.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// (Output)
    /// Detailed failure information, which might be unformatted.
    #[builder(into)]
    #[serde(rename = "details")]
    pub r#details: Option<String>,
    /// (Output)
    /// Options used for the validation check.
    #[builder(into)]
    #[serde(rename = "options")]
    pub r#options: Option<String>,
    /// (Output)
    /// A human-readable message of the check failure.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
}
