#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSQuotaInfosQuotaInfoQuotaIncreaseEligibility {
    /// The enumeration of reasons when it is ineligible to request increase adjustment.
    #[builder(into)]
    #[serde(rename = "ineligibilityReason")]
    pub r#ineligibility_reason: String,
    /// Whether a higher quota value can be requested for the quota.
    #[builder(into)]
    #[serde(rename = "isEligible")]
    pub r#is_eligible: bool,
}
