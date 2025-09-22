#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinkedServiceSnowflakeKeyVaultPassword {
    /// Specifies the name of an existing Key Vault Data Factory Linked Service.
    #[builder(into)]
    #[serde(rename = "linkedServiceName")]
    pub r#linked_service_name: String,
    /// Specifies the secret name in Azure Key Vault that stores Snowflake password.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: String,
}
