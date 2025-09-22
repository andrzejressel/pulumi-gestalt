#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobDefinitionEksPropertiesPodPropertiesContainersSecurityContext {
    #[builder(into)]
    #[serde(rename = "privileged")]
    pub r#privileged: Option<bool>,
    #[builder(into)]
    #[serde(rename = "readOnlyRootFileSystem")]
    pub r#read_only_root_file_system: Option<bool>,
    #[builder(into)]
    #[serde(rename = "runAsGroup")]
    pub r#run_as_group: Option<i32>,
    #[builder(into)]
    #[serde(rename = "runAsNonRoot")]
    pub r#run_as_non_root: Option<bool>,
    #[builder(into)]
    #[serde(rename = "runAsUser")]
    pub r#run_as_user: Option<i32>,
}
