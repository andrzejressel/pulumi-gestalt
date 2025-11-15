#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProxyAuth {
    /// The type of authentication that the proxy uses for connections from the proxy to the underlying database. One of `SECRETS`.
    #[builder(into)]
    #[serde(rename = "authScheme")]
    pub r#auth_scheme: Option<String>,
    /// The type of authentication the proxy uses for connections from clients. Valid values are `MYSQL_NATIVE_PASSWORD`, `POSTGRES_SCRAM_SHA_256`, `POSTGRES_MD5`, and `SQL_SERVER_AUTHENTICATION`.
    #[builder(into)]
    #[serde(rename = "clientPasswordAuthType")]
    pub r#client_password_auth_type: Option<String>,
    /// A user-specified description about the authentication used by a proxy to log in as a specific database user.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Whether to require or disallow AWS Identity and Access Management (IAM) authentication for connections to the proxy. One of `DISABLED`, `REQUIRED`.
    #[builder(into)]
    #[serde(rename = "iamAuth")]
    pub r#iam_auth: Option<String>,
    /// The Amazon Resource Name (ARN) representing the secret that the proxy uses to authenticate to the RDS DB instance or Aurora DB cluster. These secrets are stored within Amazon Secrets Manager.
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Option<String>,
    /// The name of the database user to which the proxy connects.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
