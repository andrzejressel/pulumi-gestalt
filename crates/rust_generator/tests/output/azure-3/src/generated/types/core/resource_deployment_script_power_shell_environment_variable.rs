#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourceDeploymentScriptPowerShellEnvironmentVariable {
    /// Specifies the name of the environment variable.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the value of the secure environment variable.
    #[builder(into, default)]
    #[serde(rename = "secureValue")]
    pub r#secure_value: Box<Option<String>>,
    /// Specifies the value of the environment variable.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
