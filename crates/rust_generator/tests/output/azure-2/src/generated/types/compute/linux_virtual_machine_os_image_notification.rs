#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxVirtualMachineOsImageNotification {
    /// Length of time a notification to be sent to the VM on the instance metadata server till the VM gets OS upgraded. The only possible value is `PT15M`. Defaults to `PT15M`.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}
