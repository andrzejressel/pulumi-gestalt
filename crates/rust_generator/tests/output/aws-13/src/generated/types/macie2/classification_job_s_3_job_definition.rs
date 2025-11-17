#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClassificationJobS3JobDefinition {
    /// The property- and tag-based conditions that determine which S3 buckets to include or exclude from the analysis. Conflicts with `bucket_definitions`. (documented below)
    #[builder(into)]
    #[serde(rename = "bucketCriteria")]
    pub r#bucket_criteria: Option<Box<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketCriteria>>,
    /// An array of objects, one for each AWS account that owns buckets to analyze. Each object specifies the account ID for an account and one or more buckets to analyze for the account. Conflicts with `bucket_criteria`. (documented below)
    #[builder(into)]
    #[serde(rename = "bucketDefinitions")]
    pub r#bucket_definitions: Option<Vec<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketDefinition>>,
    /// The property- and tag-based conditions that determine which objects to include or exclude from the analysis. (documented below)
    #[builder(into)]
    #[serde(rename = "scoping")]
    pub r#scoping: Option<Box<super::super::types::macie2::ClassificationJobS3JobDefinitionScoping>>,
}
