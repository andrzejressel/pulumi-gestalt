#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomTargetTypeCustomActionsIncludeSkaffoldModuleGoogleCloudStorage {
    /// Relative path from the source to the Skaffold file.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Cloud Storage source paths to copy recursively. For example, providing `gs://my-bucket/dir/configs/*` will result in Skaffold copying all files within the `dir/configs` directory in the bucket `my-bucket`.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: String,
}
