#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetBucketsBucket {
    /// User-provided bucket labels, in key/value pairs.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// The location of the bucket.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The name of the bucket.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A url reference to the bucket.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: String,
    /// The [StorageClass](https://cloud.google.com/storage/docs/storage-classes) of the bucket.
    #[builder(into)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: String,
}
