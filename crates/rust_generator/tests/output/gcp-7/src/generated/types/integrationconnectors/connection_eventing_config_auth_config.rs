#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionEventingConfigAuthConfig {
    /// List containing additional auth configs.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "additionalVariables")]
    pub r#additional_variables: Box<Option<Vec<super::super::types::integrationconnectors::ConnectionEventingConfigAuthConfigAdditionalVariable>>>,
    /// The type of authentication configured.
    #[builder(into, default)]
    #[serde(rename = "authKey")]
    pub r#auth_key: Box<Option<String>>,
    /// authType of the Connection
    /// Possible values are: `USER_PASSWORD`.
    #[builder(into)]
    #[serde(rename = "authType")]
    pub r#auth_type: Box<String>,
    /// User password for Authentication.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "userPassword")]
    pub r#user_password: Box<super::super::types::integrationconnectors::ConnectionEventingConfigAuthConfigUserPassword>,
}
