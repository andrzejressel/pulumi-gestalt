#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketLogging {
    /// The name of the bucket that will receive the log objects.
    #[builder(into)]
    #[serde(rename = "targetBucket")]
    pub r#target_bucket: String,
    /// To specify a key prefix for log objects.
    #[builder(into)]
    #[serde(rename = "targetPrefix")]
    pub r#target_prefix: Option<String>,
}
