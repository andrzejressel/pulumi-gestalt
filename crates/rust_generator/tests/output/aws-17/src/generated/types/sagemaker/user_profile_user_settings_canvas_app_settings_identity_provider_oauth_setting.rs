#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserProfileUserSettingsCanvasAppSettingsIdentityProviderOauthSetting {
    /// The name of the data source that you're connecting to. Canvas currently supports OAuth for Snowflake and Salesforce Data Cloud. Valid values are `SalesforceGenie` and `Snowflake`.
    #[builder(into)]
    #[serde(rename = "dataSourceName")]
    pub r#data_source_name: Option<String>,
    /// The ARN of an Amazon Web Services Secrets Manager secret that stores the credentials from your identity provider, such as the client ID and secret, authorization URL, and token URL.
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: String,
    /// Describes whether OAuth for a data source is enabled or disabled in the Canvas application. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
