#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HelmReleaseSettings {
    /// The backend storage driver for Helm. Values are: configmap, secret, memory, sql.
    #[builder(into)]
    #[serde(rename = "driver")]
    pub r#driver: Option<String>,
    /// The path to the helm plugins directory.
    #[builder(into)]
    #[serde(rename = "pluginsPath")]
    pub r#plugins_path: Option<String>,
    /// to test required args
    #[builder(into)]
    #[serde(rename = "requiredArg")]
    pub r#required_arg: String,
}
