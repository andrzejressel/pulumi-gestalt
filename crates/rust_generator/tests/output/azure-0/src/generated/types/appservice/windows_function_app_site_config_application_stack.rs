#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsFunctionAppSiteConfigApplicationStack {
    /// The version of .NET to use. Possible values include `v3.0`, `v4.0` `v6.0`, `v7.0`, `v8.0` and `v9.0`. Defaults to `v4.0`.
    #[builder(into)]
    #[serde(rename = "dotnetVersion")]
    pub r#dotnet_version: Option<String>,
    /// The Version of Java to use. Supported versions include `1.8`, `11`, `17`, `21` (In-Preview).
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Option<String>,
    /// The version of Node to run. Possible values include `~12`, `~14`, `~16`, `~18` and `~20`.
    #[builder(into)]
    #[serde(rename = "nodeVersion")]
    pub r#node_version: Option<String>,
    /// The version of PowerShell Core to run. Possible values are `7`, `7.2`, and `7.4`.
    /// 
    /// > **NOTE:** A value of `7` will provide the latest stable version. `7.2` is in preview at the time of writing.
    #[builder(into)]
    #[serde(rename = "powershellCoreVersion")]
    pub r#powershell_core_version: Option<String>,
    /// Should the Windows Function App use a custom runtime?
    #[builder(into)]
    #[serde(rename = "useCustomRuntime")]
    pub r#use_custom_runtime: Option<bool>,
    /// Should the DotNet process use an isolated runtime. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "useDotnetIsolatedRuntime")]
    pub r#use_dotnet_isolated_runtime: Option<bool>,
}
