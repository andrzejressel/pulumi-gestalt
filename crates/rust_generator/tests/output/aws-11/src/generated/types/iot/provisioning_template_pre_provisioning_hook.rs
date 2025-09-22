#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ProvisioningTemplatePreProvisioningHook {
    /// The version of the payload that was sent to the target function. The only valid (and the default) payload version is `"2020-04-01"`.
    #[builder(into)]
    #[serde(rename = "payloadVersion")]
    pub r#payload_version: Option<String>,
    /// The ARN of the target function.
    #[builder(into)]
    #[serde(rename = "targetArn")]
    pub r#target_arn: String,
}
