#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketV2LifecycleRuleExpiration {
    /// Specifies the date after which you want the corresponding action to take effect.
    #[builder(into, default)]
    #[serde(rename = "date")]
    pub r#date: Box<Option<String>>,
    /// Specifies the number of days after object creation when the specific rule action takes effect.
    #[builder(into, default)]
    #[serde(rename = "days")]
    pub r#days: Box<Option<i32>>,
    /// On a versioned bucket (versioning-enabled or versioning-suspended bucket), you can add this element in the lifecycle configuration to direct Amazon S3 to delete expired object delete markers. This cannot be specified with Days or Date in a Lifecycle Expiration Policy.
    #[builder(into, default)]
    #[serde(rename = "expiredObjectDeleteMarker")]
    pub r#expired_object_delete_marker: Box<Option<bool>>,
}
