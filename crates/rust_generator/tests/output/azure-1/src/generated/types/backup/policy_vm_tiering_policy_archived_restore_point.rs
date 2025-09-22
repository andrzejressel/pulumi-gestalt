#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyVmTieringPolicyArchivedRestorePoint {
    /// The number of days/weeks/months/years to retain backups in current tier before tiering.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Option<i32>,
    /// The retention duration type. Possible values are `Days`, `Weeks`, `Months` and `Years`.
    #[builder(into)]
    #[serde(rename = "durationType")]
    pub r#duration_type: Option<String>,
    /// The tiering mode to control automatic tiering of recovery points. Possible values are `TierAfter` and `TierRecommended`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
}
