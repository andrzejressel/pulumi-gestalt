#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntegrationRuntimeSsisExpressCustomSetup {
    /// One or more `command_key` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "commandKeys")]
    pub r#command_keys: Option<Vec<super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetupCommandKey>>,
    /// One or more `component` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Option<Vec<super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetupComponent>>,
    /// The Environment Variables for the Azure-SSIS Integration Runtime.
    #[builder(into)]
    #[serde(rename = "environment")]
    pub r#environment: Option<std::collections::HashMap<String, String>>,
    /// The version of Azure Powershell installed for the Azure-SSIS Integration Runtime.
    /// 
    /// > **NOTE** At least one of `env`, `powershell_version`, `component` and `command_key` should be specified.
    #[builder(into)]
    #[serde(rename = "powershellVersion")]
    pub r#powershell_version: Option<String>,
}
