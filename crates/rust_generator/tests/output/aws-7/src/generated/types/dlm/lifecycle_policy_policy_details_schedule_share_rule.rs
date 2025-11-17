#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetailsScheduleShareRule {
    /// The IDs of the AWS accounts with which to share the snapshots.
    #[builder(into)]
    #[serde(rename = "targetAccounts")]
    pub r#target_accounts: Vec<String>,
    /// The period after which snapshots that are shared with other AWS accounts are automatically unshared.
    #[builder(into)]
    #[serde(rename = "unshareInterval")]
    pub r#unshare_interval: Option<i32>,
    /// The unit of time for the automatic unsharing interval. Valid values are `DAYS`, `WEEKS`, `MONTHS`, `YEARS`.
    #[builder(into)]
    #[serde(rename = "unshareIntervalUnit")]
    pub r#unshare_interval_unit: Option<String>,
}
