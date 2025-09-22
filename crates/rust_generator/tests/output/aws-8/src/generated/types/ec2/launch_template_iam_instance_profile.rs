#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LaunchTemplateIamInstanceProfile {
    /// The Amazon Resource Name (ARN) of the instance profile. Conflicts with `name`.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Option<String>,
    /// The name of the instance profile.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
