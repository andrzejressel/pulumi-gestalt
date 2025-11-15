#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HadoopClusterRolesEdgeNodeUninstallScriptAction {
    /// The name of the uninstall script action.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The parameters for the script.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<String>,
    /// The URI pointing to the script to run during the installation of the edge node.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}
