#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBucketLogging {
    /// The bucket that will receive log objects.
    #[builder(into)]
    #[serde(rename = "logBucket")]
    pub r#log_bucket: String,
    /// The object prefix for log objects. If it's not provided, by default Google Cloud Storage sets this to this bucket's name.
    #[builder(into)]
    #[serde(rename = "logObjectPrefix")]
    pub r#log_object_prefix: String,
}
