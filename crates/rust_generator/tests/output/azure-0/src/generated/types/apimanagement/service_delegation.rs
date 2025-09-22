#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceDelegation {
    /// Should subscription requests be delegated to an external url? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "subscriptionsEnabled")]
    pub r#subscriptions_enabled: Option<bool>,
    /// The delegation URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// Should user registration requests be delegated to an external url? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "userRegistrationEnabled")]
    pub r#user_registration_enabled: Option<bool>,
    /// A base64-encoded validation key to validate, that a request is coming from Azure API Management.
    #[builder(into)]
    #[serde(rename = "validationKey")]
    pub r#validation_key: Option<String>,
}
