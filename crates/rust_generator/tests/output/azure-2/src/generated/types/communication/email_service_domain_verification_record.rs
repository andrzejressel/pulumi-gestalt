#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EmailServiceDomainVerificationRecord {
    /// (Optional) An `dkim2` block as defined below.
    #[builder(into)]
    #[serde(rename = "dkim2s")]
    pub r#dkim_2_s: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDkim2>>,
    /// (Optional) An `dkim` block as defined below.
    #[builder(into)]
    #[serde(rename = "dkims")]
    pub r#dkims: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDkim>>,
    /// (Optional) An `dmarc` block as defined below.
    #[builder(into)]
    #[serde(rename = "dmarcs")]
    pub r#dmarcs: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDmarc>>,
    /// (Optional) An `domain` block as defined below.
    #[builder(into)]
    #[serde(rename = "domains")]
    pub r#domains: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordDomain>>,
    /// (Optional) An `spf` block as defined below.
    #[builder(into)]
    #[serde(rename = "spfs")]
    pub r#spfs: Option<Vec<super::super::types::communication::EmailServiceDomainVerificationRecordSpf>>,
}
