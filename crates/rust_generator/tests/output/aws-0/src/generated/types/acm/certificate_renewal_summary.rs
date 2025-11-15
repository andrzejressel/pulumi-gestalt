#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateRenewalSummary {
    /// The status of ACM's managed renewal of the certificate
    #[builder(into)]
    #[serde(rename = "renewalStatus")]
    pub r#renewal_status: Option<String>,
    /// The reason that a renewal request was unsuccessful or is pending
    #[builder(into)]
    #[serde(rename = "renewalStatusReason")]
    pub r#renewal_status_reason: Option<String>,
    #[builder(into)]
    #[serde(rename = "updatedAt")]
    pub r#updated_at: Option<String>,
}
