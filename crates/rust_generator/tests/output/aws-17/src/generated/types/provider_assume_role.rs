#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderAssumeRole {
    /// The duration, between 15 minutes and 12 hours, of the role session. Valid time units are ns, us (or µs), ms, s, h, or m.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Option<String>,
    /// A unique identifier that might be required when you assume a role in another account.
    #[builder(into)]
    #[serde(rename = "externalId")]
    pub r#external_id: Option<String>,
    /// IAM Policy JSON describing further restricting permissions for the IAM Role being assumed.
    #[builder(into)]
    #[serde(rename = "policy")]
    pub r#policy: Option<String>,
    /// Amazon Resource Names (ARNs) of IAM Policies describing further restricting permissions for the IAM Role being assumed.
    #[builder(into)]
    #[serde(rename = "policyArns")]
    pub r#policy_arns: Option<Vec<String>>,
    /// Amazon Resource Name (ARN) of an IAM Role to assume prior to making API calls.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Option<String>,
    /// An identifier for the assumed role session.
    #[builder(into)]
    #[serde(rename = "sessionName")]
    pub r#session_name: Option<String>,
    /// Source identity specified by the principal assuming the role.
    #[builder(into)]
    #[serde(rename = "sourceIdentity")]
    pub r#source_identity: Option<String>,
    /// Assume role session tags.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// Assume role session tag keys to pass to any subsequent sessions.
    #[builder(into)]
    #[serde(rename = "transitiveTagKeys")]
    pub r#transitive_tag_keys: Option<Vec<String>>,
}
