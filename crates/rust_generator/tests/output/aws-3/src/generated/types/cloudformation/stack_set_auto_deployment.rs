#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StackSetAutoDeployment {
    /// Whether or not auto-deployment is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Whether or not to retain stacks when the account is removed.
    #[builder(into)]
    #[serde(rename = "retainStacksOnAccountRemoval")]
    pub r#retain_stacks_on_account_removal: Option<bool>,
}
