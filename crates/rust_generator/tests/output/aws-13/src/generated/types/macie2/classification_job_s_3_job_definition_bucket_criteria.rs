#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClassificationJobS3JobDefinitionBucketCriteria {
    /// The property- or tag-based conditions that determine which S3 buckets to exclude from the analysis. (documented below)
    #[builder(into)]
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketCriteriaExcludes>>,
    /// The property- or tag-based conditions that determine which S3 buckets to include in the analysis. (documented below)
    #[builder(into)]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketCriteriaIncludes>>,
}
