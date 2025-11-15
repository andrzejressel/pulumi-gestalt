#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TemplateSourceEntity {
    /// The source analysis, if it is based on an analysis.. Only one of `source_analysis` or `source_template` should be configured. See source_analysis.
    #[builder(into)]
    #[serde(rename = "sourceAnalysis")]
    pub r#source_analysis: Option<Box<super::super::types::quicksight::TemplateSourceEntitySourceAnalysis>>,
    /// The source template, if it is based on an template.. Only one of `source_analysis` or `source_template` should be configured. See source_template.
    #[builder(into)]
    #[serde(rename = "sourceTemplate")]
    pub r#source_template: Option<Box<super::super::types::quicksight::TemplateSourceEntitySourceTemplate>>,
}
