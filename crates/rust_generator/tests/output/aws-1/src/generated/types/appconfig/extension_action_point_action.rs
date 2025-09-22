#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExtensionActionPointAction {
    /// Information about the action.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The action name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// An Amazon Resource Name (ARN) for an Identity and Access Management assume role.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Option<String>,
    /// The extension URI associated to the action point in the extension definition. The URI can be an Amazon Resource Name (ARN) for one of the following: an Lambda function, an Amazon Simple Queue Service queue, an Amazon Simple Notification Service topic, or the Amazon EventBridge default event bus.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}
