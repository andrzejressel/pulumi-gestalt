#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DirectoryBucketLocation {
    /// [Availability Zone ID](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html#az-ids) or Local Zone ID.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Location type. Valid values: `AvailabilityZone`, `LocalZone`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
