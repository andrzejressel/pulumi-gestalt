#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DirectoryWorkspaceAccessProperties {
    /// Indicates whether users can use Android devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeAndroid")]
    pub r#device_type_android: Option<String>,
    /// Indicates whether users can use Chromebooks to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeChromeos")]
    pub r#device_type_chromeos: Option<String>,
    /// Indicates whether users can use iOS devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeIos")]
    pub r#device_type_ios: Option<String>,
    /// Indicates whether users can use Linux clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeLinux")]
    pub r#device_type_linux: Option<String>,
    /// Indicates whether users can use macOS clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeOsx")]
    pub r#device_type_osx: Option<String>,
    /// Indicates whether users can access their WorkSpaces through a web browser.
    #[builder(into)]
    #[serde(rename = "deviceTypeWeb")]
    pub r#device_type_web: Option<String>,
    /// Indicates whether users can use Windows clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeWindows")]
    pub r#device_type_windows: Option<String>,
    /// Indicates whether users can use zero client devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeZeroclient")]
    pub r#device_type_zeroclient: Option<String>,
}
