#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudBuildPackBindingLaunch {
    /// Specifies a map of non-sensitive properties for launchProperties.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
    /// Specifies a map of sensitive properties for launchProperties.
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Option<std::collections::HashMap<String, String>>,
}
