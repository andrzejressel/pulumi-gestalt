#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFunctionFileSystemConfig {
    /// Unqualified (no `:QUALIFIER` or `:VERSION` suffix) ARN identifying your Lambda Function. See also `qualified_arn`.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "localMountPath")]
    pub r#local_mount_path: Box<String>,
}
