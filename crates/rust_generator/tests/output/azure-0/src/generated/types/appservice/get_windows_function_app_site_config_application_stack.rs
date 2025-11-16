#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWindowsFunctionAppSiteConfigApplicationStack {
    /// The version of .Net to use.
    #[builder(into)]
    #[serde(rename = "dotnetVersion")]
    pub r#dotnet_version: String,
    /// The version of Java to use.
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: String,
    /// The version of Node to use.
    #[builder(into)]
    #[serde(rename = "nodeVersion")]
    pub r#node_version: String,
    /// The version of PowerShell Core to use.
    #[builder(into)]
    #[serde(rename = "powershellCoreVersion")]
    pub r#powershell_core_version: String,
    /// Is the Windows Function App using a custom runtime?.
    #[builder(into)]
    #[serde(rename = "useCustomRuntime")]
    pub r#use_custom_runtime: bool,
    #[builder(into)]
    #[serde(rename = "useDotnetIsolatedRuntime")]
    pub r#use_dotnet_isolated_runtime: bool,
}
