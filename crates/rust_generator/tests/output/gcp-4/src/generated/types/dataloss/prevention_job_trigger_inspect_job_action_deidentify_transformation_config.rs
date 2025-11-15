#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobActionDeidentifyTransformationConfig {
    /// If this template is specified, it will serve as the default de-identify template.
    #[builder(into)]
    #[serde(rename = "deidentifyTemplate")]
    pub r#deidentify_template: Option<String>,
    /// If this template is specified, it will serve as the de-identify template for images.
    #[builder(into)]
    #[serde(rename = "imageRedactTemplate")]
    pub r#image_redact_template: Option<String>,
    /// If this template is specified, it will serve as the de-identify template for structured content such as delimited files and tables.
    #[builder(into)]
    #[serde(rename = "structuredDeidentifyTemplate")]
    pub r#structured_deidentify_template: Option<String>,
}
