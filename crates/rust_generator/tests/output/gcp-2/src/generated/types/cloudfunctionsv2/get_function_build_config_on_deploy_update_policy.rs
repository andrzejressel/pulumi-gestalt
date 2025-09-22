#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFunctionBuildConfigOnDeployUpdatePolicy {
    /// The runtime version which was used during latest function deployment.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: String,
}
