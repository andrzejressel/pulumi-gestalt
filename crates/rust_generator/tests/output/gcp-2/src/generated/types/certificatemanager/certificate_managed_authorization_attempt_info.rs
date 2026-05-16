#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateManagedAuthorizationAttemptInfo {
    /// Human readable explanation for reaching the state. Provided to help
    /// address the configuration issues.
    /// Not guaranteed to be stable. For programmatic access use 'failure_reason' field.
    #[builder(into)]
    #[serde(rename = "details")]
    pub r#details: Option<String>,
    /// Domain name of the authorization attempt.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Option<String>,
    /// Reason for failure of the authorization attempt for the domain.
    #[builder(into)]
    #[serde(rename = "failureReason")]
    pub r#failure_reason: Option<String>,
    /// State of the domain for managed certificate issuance.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}
