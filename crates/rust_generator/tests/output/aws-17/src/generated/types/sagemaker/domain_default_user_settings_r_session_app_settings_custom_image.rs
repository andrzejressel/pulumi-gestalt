#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainDefaultUserSettingsRSessionAppSettingsCustomImage {
    /// The name of the App Image Config.
    #[builder(into)]
    #[serde(rename = "appImageConfigName")]
    pub r#app_image_config_name: Box<String>,
    /// The name of the Custom Image.
    #[builder(into)]
    #[serde(rename = "imageName")]
    pub r#image_name: Box<String>,
    /// The version number of the Custom Image.
    #[builder(into, default)]
    #[serde(rename = "imageVersionNumber")]
    pub r#image_version_number: Box<Option<i32>>,
}
