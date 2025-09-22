#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ProjectCache {
    /// Location where the AWS CodeBuild project stores cached resources. For type `S3`, the value must be a valid S3 bucket name/prefix.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// Specifies settings that AWS CodeBuild uses to store and reuse build dependencies. Valid values:  `LOCAL_SOURCE_CACHE`, `LOCAL_DOCKER_LAYER_CACHE`, `LOCAL_CUSTOM_CACHE`.
    #[builder(into)]
    #[serde(rename = "modes")]
    pub r#modes: Option<Vec<String>>,
    /// Type of storage that will be used for the AWS CodeBuild project cache. Valid values: `NO_CACHE`, `LOCAL`, `S3`. Defaults to `NO_CACHE`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
