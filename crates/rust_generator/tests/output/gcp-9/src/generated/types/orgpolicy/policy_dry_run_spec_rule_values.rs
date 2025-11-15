#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyDryRunSpecRuleValues {
    /// List of values allowed at this resource.
    #[builder(into)]
    #[serde(rename = "allowedValues")]
    pub r#allowed_values: Option<Vec<String>>,
    /// List of values denied at this resource.
    #[builder(into)]
    #[serde(rename = "deniedValues")]
    pub r#denied_values: Option<Vec<String>>,
}
