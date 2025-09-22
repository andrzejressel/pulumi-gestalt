#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobDefinitionEksPropertiesPodPropertiesContainersResources {
    #[builder(into)]
    #[serde(rename = "limits")]
    pub r#limits: Option<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "requests")]
    pub r#requests: Option<std::collections::HashMap<String, String>>,
}
