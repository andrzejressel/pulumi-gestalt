#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppTemplateContainerEnv {
    /// The name of the environment variable for the container.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the secret that contains the value for this environment variable.
    #[builder(into, default)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<Option<String>>,
    /// The value for this environment variable.
    /// 
    /// > **NOTE:** This value is ignored if `secret_name` is used
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
