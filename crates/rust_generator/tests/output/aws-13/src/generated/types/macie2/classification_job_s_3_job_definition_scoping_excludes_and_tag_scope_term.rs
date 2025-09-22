#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTerm {
    /// The operator to use in the condition.
    #[builder(into)]
    #[serde(rename = "comparator")]
    pub r#comparator: Option<String>,
    /// The tag key to use in the condition. The only valid value is `TAG`.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The tag keys or tag key and value pairs to use in the condition.
    #[builder(into)]
    #[serde(rename = "tagValues")]
    pub r#tag_values: Option<Vec<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingExcludesAndTagScopeTermTagValue>>,
    /// The type of object to apply the condition to. The only valid value is `S3_OBJECT`.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<String>,
}
