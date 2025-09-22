#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLaunchTemplateIamInstanceProfile {
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// Name of the launch template.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
