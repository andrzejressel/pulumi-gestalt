#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkInterfaceAttachment {
    #[builder(into)]
    #[serde(rename = "attachmentId")]
    pub r#attachment_id: Option<String>,
    /// Integer to define the devices index.
    #[builder(into)]
    #[serde(rename = "deviceIndex")]
    pub r#device_index: i32,
    /// ID of the instance to attach to.
    #[builder(into)]
    #[serde(rename = "instance")]
    pub r#instance: String,
}
