#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketLifecycleConfigurationV2RuleFilter {
    /// Configuration block used to apply a logical `AND` to two or more predicates. See below. The Lifecycle Rule will apply to any object matching all the predicates configured inside the `and` block.
    #[builder(into)]
    #[serde(rename = "and")]
    pub r#and: Box<Option<super::super::types::s3::BucketLifecycleConfigurationV2RuleFilterAnd>>,
    /// Minimum object size (in bytes) to which the rule applies.
    #[builder(into)]
    #[serde(rename = "objectSizeGreaterThan")]
    pub r#object_size_greater_than: Option<String>,
    /// Maximum object size (in bytes) to which the rule applies.
    #[builder(into)]
    #[serde(rename = "objectSizeLessThan")]
    pub r#object_size_less_than: Option<String>,
    /// Prefix identifying one or more objects to which the rule applies. Defaults to an empty string (`""`) if not specified.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Configuration block for specifying a tag key and value. See below.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<super::super::types::s3::BucketLifecycleConfigurationV2RuleFilterTag>>,
}
