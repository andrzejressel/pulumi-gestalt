#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClassificationJobS3JobDefinitionScoping {
    /// The property- or tag-based conditions that determine which objects to exclude from the analysis. (documented below)
    #[builder(into)]
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Box<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingExcludes>>,
    /// The property- or tag-based conditions that determine which objects to include in the analysis. (documented below)
    #[builder(into)]
    #[serde(rename = "includes")]
    pub r#includes: Option<Box<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingIncludes>>,
}
