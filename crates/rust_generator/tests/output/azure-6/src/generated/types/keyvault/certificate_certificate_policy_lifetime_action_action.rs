#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateCertificatePolicyLifetimeActionAction {
    /// The Type of action to be performed when the lifetime trigger is triggerec. Possible values include `AutoRenew` and `EmailContacts`.
    #[builder(into)]
    #[serde(rename = "actionType")]
    pub r#action_type: Box<String>,
}
