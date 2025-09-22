#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterion {
    /// The operator to use in the condition. Valid combination and values are available in the [AWS Documentation](https://docs.aws.amazon.com/macie/latest/APIReference/jobs.html#jobs-model-jobcomparator)
    #[builder(into)]
    #[serde(rename = "comparator")]
    pub r#comparator: Option<String>,
    /// The  tag key and value pairs to use in the condition. One or more blocks are allowed. (documented below)
    #[builder(into)]
    #[serde(rename = "tagValues")]
    pub r#tag_values: Option<Vec<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketCriteriaIncludesAndTagCriterionTagValue>>,
}
