#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LaunchTemplateHibernationOptions {
    /// If set to `true`, the launched EC2 instance will hibernation enabled.
    #[builder(into)]
    #[serde(rename = "configured")]
    pub r#configured: bool,
}
