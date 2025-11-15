#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineConfigurationAssignmentConfiguration {
    /// The assignment type for the Guest Configuration Assignment. Possible values are `Audit`, `ApplyAndAutoCorrect`, `ApplyAndMonitor` and `DeployAndAutoCorrect`.
    #[builder(into)]
    #[serde(rename = "assignmentType")]
    pub r#assignment_type: Option<String>,
    /// The content hash for the Guest Configuration package.
    #[builder(into)]
    #[serde(rename = "contentHash")]
    pub r#content_hash: Option<String>,
    /// The content URI where the Guest Configuration package is stored.
    /// 
    /// > **NOTE:** When deploying a Custom Guest Configuration package the `content_hash` and `content_uri` fields must be defined. For Built-in Guest Configuration packages, such as the `AzureWindowsBaseline` package, the `content_hash` and `content_uri` should not be defined, rather these fields will be returned after the Built-in Guest Configuration package has been provisioned. For more information on guest configuration assignments please see the [product documentation](https://docs.microsoft.com/azure/governance/policy/concepts/guest-configuration-assignments).
    #[builder(into)]
    #[serde(rename = "contentUri")]
    pub r#content_uri: Option<String>,
    /// One or more `parameter` blocks as defined below which define what configuration parameters and values against.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::policy::VirtualMachineConfigurationAssignmentConfigurationParameter>>,
    /// The version of the Guest Configuration that will be assigned in this Guest Configuration Assignment.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
