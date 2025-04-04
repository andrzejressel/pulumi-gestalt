#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationPortalOptionsSignInOptions {
    /// URL that accepts authentication requests for an application.
    #[builder(into, default)]
    #[serde(rename = "applicationUrl")]
    pub r#application_url: Box<Option<String>>,
    /// Determines how IAM Identity Center navigates the user to the target application.
    /// Valid values are `APPLICATION` and `IDENTITY_CENTER`.
    /// If `APPLICATION` is set, IAM Identity Center redirects the customer to the configured `application_url`.
    /// If `IDENTITY_CENTER` is set, IAM Identity Center uses SAML identity-provider initiated authentication to sign the customer directly into a SAML-based application.
    #[builder(into)]
    #[serde(rename = "origin")]
    pub r#origin: Box<String>,
}
