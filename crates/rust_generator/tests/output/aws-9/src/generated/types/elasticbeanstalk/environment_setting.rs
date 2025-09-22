#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnvironmentSetting {
    /// A unique name for this Environment. This name is used
    /// in the application URL
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: String,
    #[builder(into)]
    #[serde(rename = "resource")]
    pub r#resource: Option<String>,
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
