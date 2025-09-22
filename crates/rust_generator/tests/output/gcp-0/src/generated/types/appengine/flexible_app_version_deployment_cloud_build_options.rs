#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleAppVersionDeploymentCloudBuildOptions {
    /// Path to the yaml file used in deployment, used to determine runtime configuration details.
    #[builder(into)]
    #[serde(rename = "appYamlPath")]
    pub r#app_yaml_path: String,
    /// The Cloud Build timeout used as part of any dependent builds performed by version creation. Defaults to 10 minutes.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "cloudBuildTimeout")]
    pub r#cloud_build_timeout: Option<String>,
}
