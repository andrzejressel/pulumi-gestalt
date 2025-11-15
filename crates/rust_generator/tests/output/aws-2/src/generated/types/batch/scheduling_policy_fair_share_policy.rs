#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SchedulingPolicyFairSharePolicy {
    /// A value used to reserve some of the available maximum vCPU for fair share identifiers that have not yet been used. For more information, see [FairsharePolicy](https://docs.aws.amazon.com/batch/latest/APIReference/API_FairsharePolicy.html).
    #[builder(into)]
    #[serde(rename = "computeReservation")]
    pub r#compute_reservation: Option<i32>,
    #[builder(into)]
    #[serde(rename = "shareDecaySeconds")]
    pub r#share_decay_seconds: Option<i32>,
    /// One or more share distribution blocks which define the weights for the fair share identifiers for the fair share policy. For more information, see [FairsharePolicy](https://docs.aws.amazon.com/batch/latest/APIReference/API_FairsharePolicy.html). The `share_distribution` block is documented below.
    #[builder(into)]
    #[serde(rename = "shareDistributions")]
    pub r#share_distributions: Option<Vec<super::super::types::batch::SchedulingPolicyFairSharePolicyShareDistribution>>,
}
