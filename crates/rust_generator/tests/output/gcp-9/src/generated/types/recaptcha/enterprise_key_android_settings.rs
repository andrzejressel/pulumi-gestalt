#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnterpriseKeyAndroidSettings {
    /// If set to true, it means allowed_package_names will not be enforced.
    #[builder(into)]
    #[serde(rename = "allowAllPackageNames")]
    pub r#allow_all_package_names: Option<bool>,
    /// Android package names of apps allowed to use the key. Example: 'com.companyname.appname'
    #[builder(into)]
    #[serde(rename = "allowedPackageNames")]
    pub r#allowed_package_names: Option<Vec<String>>,
}
