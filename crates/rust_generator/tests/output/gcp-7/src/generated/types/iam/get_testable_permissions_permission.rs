#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTestablePermissionsPermission {
    /// Whether the corresponding API has been enabled for the resource.
    #[builder(into)]
    #[serde(rename = "apiDisabled")]
    pub r#api_disabled: bool,
    /// The level of support for custom roles. Can be one of `"NOT_SUPPORTED"`, `"SUPPORTED"`, `"TESTING"`. Default is `"SUPPORTED"`
    #[builder(into)]
    #[serde(rename = "customSupportLevel")]
    pub r#custom_support_level: String,
    /// Name of the permission.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Release stage of the permission.
    #[builder(into)]
    #[serde(rename = "stage")]
    pub r#stage: String,
    /// Human readable title of the permission.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: String,
}
