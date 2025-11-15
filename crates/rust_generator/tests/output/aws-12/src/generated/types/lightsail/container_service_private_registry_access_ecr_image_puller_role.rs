#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContainerServicePrivateRegistryAccessEcrImagePullerRole {
    /// A Boolean value that indicates whether to activate the role. The default is `false`.
    #[builder(into)]
    #[serde(rename = "isActive")]
    pub r#is_active: Option<bool>,
    /// The principal ARN of the container service. The principal ARN can be used to create a trust
    /// relationship between your standard AWS account and your Lightsail container service. This allows you to give your
    /// service permission to access resources in your standard AWS account.
    #[builder(into)]
    #[serde(rename = "principalArn")]
    pub r#principal_arn: Option<String>,
}
