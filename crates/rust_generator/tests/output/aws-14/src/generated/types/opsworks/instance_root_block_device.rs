#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceRootBlockDevice {
    #[builder(into)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Option<bool>,
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Option<i32>,
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Option<i32>,
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Option<String>,
}
