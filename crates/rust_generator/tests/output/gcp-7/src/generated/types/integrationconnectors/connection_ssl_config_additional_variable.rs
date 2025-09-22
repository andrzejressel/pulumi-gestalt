#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionSslConfigAdditionalVariable {
    /// Boolean Value of configVariable.
    #[builder(into)]
    #[serde(rename = "booleanValue")]
    pub r#boolean_value: Option<bool>,
    /// Encryption key value of configVariable.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "encryptionKeyValue")]
    pub r#encryption_key_value: Box<Option<super::super::types::integrationconnectors::ConnectionSslConfigAdditionalVariableEncryptionKeyValue>>,
    /// Integer Value of configVariable.
    #[builder(into)]
    #[serde(rename = "integerValue")]
    pub r#integer_value: Option<i32>,
    /// Key for the configVariable
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Secret value of configVariable
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secretValue")]
    pub r#secret_value: Box<Option<super::super::types::integrationconnectors::ConnectionSslConfigAdditionalVariableSecretValue>>,
    /// String Value of configVariabley.
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Option<String>,
}
