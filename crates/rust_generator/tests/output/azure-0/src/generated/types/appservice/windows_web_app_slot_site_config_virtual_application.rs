#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsWebAppSlotSiteConfigVirtualApplication {
    /// The physical path for the Virtual Application.
    #[builder(into)]
    #[serde(rename = "physicalPath")]
    pub r#physical_path: String,
    /// Should pre-loading be enabled.
    #[builder(into)]
    #[serde(rename = "preload")]
    pub r#preload: bool,
    /// One or more `virtual_directory` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "virtualDirectories")]
    pub r#virtual_directories: Option<Vec<super::super::types::appservice::WindowsWebAppSlotSiteConfigVirtualApplicationVirtualDirectory>>,
    /// The Virtual Path for the Virtual Application.
    #[builder(into)]
    #[serde(rename = "virtualPath")]
    pub r#virtual_path: String,
}
