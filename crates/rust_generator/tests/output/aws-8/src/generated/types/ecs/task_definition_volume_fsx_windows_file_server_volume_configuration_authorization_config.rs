#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskDefinitionVolumeFsxWindowsFileServerVolumeConfigurationAuthorizationConfig {
    /// The authorization credential option to use. The authorization credential options can be provided using either the Amazon Resource Name (ARN) of an AWS Secrets Manager secret or AWS Systems Manager Parameter Store parameter. The ARNs refer to the stored credentials.
    #[builder(into)]
    #[serde(rename = "credentialsParameter")]
    pub r#credentials_parameter: String,
    /// A fully qualified domain name hosted by an AWS Directory Service Managed Microsoft AD (Active Directory) or self-hosted AD on Amazon EC2.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: String,
}
