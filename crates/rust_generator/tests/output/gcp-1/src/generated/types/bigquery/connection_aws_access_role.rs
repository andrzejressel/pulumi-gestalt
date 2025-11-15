#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionAwsAccessRole {
    /// The user’s AWS IAM Role that trusts the Google-owned AWS IAM user Connection.
    #[builder(into)]
    #[serde(rename = "iamRoleId")]
    pub r#iam_role_id: String,
    /// (Output)
    /// A unique Google-owned and Google-generated identity for the Connection. This identity will be used to access the user's AWS IAM Role.
    #[builder(into)]
    #[serde(rename = "identity")]
    pub r#identity: Option<String>,
}
