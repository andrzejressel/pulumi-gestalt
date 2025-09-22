#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketLifecycleConfigurationV2RuleExpiration {
    /// Date the object is to be moved or deleted. The date value must be in [RFC3339 full-date format](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) e.g. `2023-08-22`.
    #[builder(into)]
    #[serde(rename = "date")]
    pub r#date: Option<String>,
    /// Lifetime, in days, of the objects that are subject to the rule. The value must be a non-zero positive integer.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: Option<i32>,
    /// Indicates whether Amazon S3 will remove a delete marker with no noncurrent versions. If set to `true`, the delete marker will be expired; if set to `false` the policy takes no action.
    #[builder(into)]
    #[serde(rename = "expiredObjectDeleteMarker")]
    pub r#expired_object_delete_marker: Option<bool>,
}
