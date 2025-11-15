#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstancePscConfig {
    /// List of VPCs that are allowed ingress into the Looker instance.
    #[builder(into)]
    #[serde(rename = "allowedVpcs")]
    pub r#allowed_vpcs: Option<Vec<String>>,
    /// (Output)
    /// URI of the Looker service attachment.
    #[builder(into)]
    #[serde(rename = "lookerServiceAttachmentUri")]
    pub r#looker_service_attachment_uri: Option<String>,
    /// List of egress service attachment configurations.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serviceAttachments")]
    pub r#service_attachments: Option<Vec<super::super::types::looker::InstancePscConfigServiceAttachment>>,
}
