#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnvironmentStorageConfiguration {
    #[builder(into)]
    #[serde(rename = "efs")]
    pub r#efs: Box<Option<super::super::types::m2::EnvironmentStorageConfigurationEfs>>,
    #[builder(into)]
    #[serde(rename = "fsx")]
    pub r#fsx: Box<Option<super::super::types::m2::EnvironmentStorageConfigurationFsx>>,
}
