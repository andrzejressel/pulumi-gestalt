#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentTargetImport {
    /// The full contents of the template that you want to import.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// The name of the template to import, as declared in the YAML
    /// configuration.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
