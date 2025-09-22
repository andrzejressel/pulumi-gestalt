#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceCredentials {
    /// The Amazon Resource Name (ARN) of a data source that has the credential pair that you want to use.
    /// When the value is not null, the `credential_pair` from the data source in the ARN is used.
    #[builder(into)]
    #[serde(rename = "copySourceArn")]
    pub r#copy_source_arn: Option<String>,
    /// Credential pair. See Credential Pair below for more details.
    #[builder(into)]
    #[serde(rename = "credentialPair")]
    pub r#credential_pair: Option<Box<super::super::types::quicksight::DataSourceCredentialsCredentialPair>>,
    /// The Amazon Resource Name (ARN) of the secret associated with the data source in Amazon Secrets Manager.
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Option<String>,
}
