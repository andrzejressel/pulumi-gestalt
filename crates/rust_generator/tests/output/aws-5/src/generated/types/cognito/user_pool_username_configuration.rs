#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPoolUsernameConfiguration {
    /// Whether username case sensitivity will be applied for all users in the user pool through Cognito APIs.
    #[builder(into)]
    #[serde(rename = "caseSensitive")]
    pub r#case_sensitive: bool,
}
