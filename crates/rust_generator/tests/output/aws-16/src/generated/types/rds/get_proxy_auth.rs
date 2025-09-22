#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetProxyAuth {
    #[builder(into)]
    #[serde(rename = "authScheme")]
    pub r#auth_scheme: String,
    #[builder(into)]
    #[serde(rename = "clientPasswordAuthType")]
    pub r#client_password_auth_type: String,
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    #[builder(into)]
    #[serde(rename = "iamAuth")]
    pub r#iam_auth: String,
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: String,
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}
