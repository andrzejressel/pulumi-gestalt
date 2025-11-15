#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDirectoryWorkspaceAccessProperty {
    /// (Optional) Indicates whether users can use Android devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeAndroid")]
    pub r#device_type_android: String,
    /// (Optional) Indicates whether users can use Chromebooks to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeChromeos")]
    pub r#device_type_chromeos: String,
    /// (Optional) Indicates whether users can use iOS devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeIos")]
    pub r#device_type_ios: String,
    /// (Optional) Indicates whether users can use Linux clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeLinux")]
    pub r#device_type_linux: String,
    /// (Optional) Indicates whether users can use macOS clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeOsx")]
    pub r#device_type_osx: String,
    /// (Optional) Indicates whether users can access their WorkSpaces through a web browser.
    #[builder(into)]
    #[serde(rename = "deviceTypeWeb")]
    pub r#device_type_web: String,
    /// (Optional) Indicates whether users can use Windows clients to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeWindows")]
    pub r#device_type_windows: String,
    /// (Optional) Indicates whether users can use zero client devices to access their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "deviceTypeZeroclient")]
    pub r#device_type_zeroclient: String,
}
