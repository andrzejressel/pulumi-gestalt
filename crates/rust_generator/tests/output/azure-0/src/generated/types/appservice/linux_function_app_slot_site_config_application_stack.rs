#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxFunctionAppSlotSiteConfigApplicationStack {
    /// a `docker` block as detailed below.
    #[builder(into)]
    #[serde(rename = "dockers")]
    pub r#dockers: Option<Vec<super::super::types::appservice::LinuxFunctionAppSlotSiteConfigApplicationStackDocker>>,
    /// The version of .Net. Possible values are `3.1`, `6.0`, `7.0`, `8.0` and `9.0`.
    #[builder(into)]
    #[serde(rename = "dotnetVersion")]
    pub r#dotnet_version: Option<String>,
    /// The version of Java to use. Possible values are `8`, `11` & `17` (In-Preview).
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Option<String>,
    /// The version of Node to use. Possible values include `12`, `14`, `16`, `18` and `20`
    #[builder(into)]
    #[serde(rename = "nodeVersion")]
    pub r#node_version: Option<String>,
    /// The version of PowerShell Core to use. Possibles values are `7` , `7.2`, and `7.4`.
    #[builder(into)]
    #[serde(rename = "powershellCoreVersion")]
    pub r#powershell_core_version: Option<String>,
    /// The version of Python to use. Possible values are `3.12`, `3.11`, `3.10`, `3.9`, `3.8` and `3.7`.
    #[builder(into)]
    #[serde(rename = "pythonVersion")]
    pub r#python_version: Option<String>,
    /// Should the Linux Function App use a custom runtime?
    #[builder(into)]
    #[serde(rename = "useCustomRuntime")]
    pub r#use_custom_runtime: Option<bool>,
    /// Should the DotNet process use an isolated runtime. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "useDotnetIsolatedRuntime")]
    pub r#use_dotnet_isolated_runtime: Option<bool>,
}
