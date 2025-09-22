#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegistryTaskPlatform {
    /// The OS architecture. Possible values are `amd64`, `x86`, `386`, `arm` and `arm64`.
    #[builder(into)]
    #[serde(rename = "architecture")]
    pub r#architecture: Option<String>,
    /// The operating system type required for the task. Possible values are `Windows` and `Linux`.
    #[builder(into)]
    #[serde(rename = "os")]
    pub r#os: String,
    /// The variant of the CPU. Possible values are `v6`, `v7`, `v8`.
    #[builder(into)]
    #[serde(rename = "variant")]
    pub r#variant: Option<String>,
}
