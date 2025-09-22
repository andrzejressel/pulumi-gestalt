#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketLifecycleConfigurationV2RuleFilterAnd {
    /// Minimum object size to which the rule applies. Value must be at least `0` if specified. Defaults to 128000 (128 KB) for all `storage_class` values unless `transition_default_minimum_object_size` specifies otherwise.
    #[builder(into)]
    #[serde(rename = "objectSizeGreaterThan")]
    pub r#object_size_greater_than: Option<i32>,
    /// Maximum object size to which the rule applies. Value must be at least `1` if specified.
    #[builder(into)]
    #[serde(rename = "objectSizeLessThan")]
    pub r#object_size_less_than: Option<i32>,
    /// Prefix identifying one or more objects to which the rule applies.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Key-value map of resource tags. All of these tags must exist in the object's tag set in order for the rule to apply.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
}
