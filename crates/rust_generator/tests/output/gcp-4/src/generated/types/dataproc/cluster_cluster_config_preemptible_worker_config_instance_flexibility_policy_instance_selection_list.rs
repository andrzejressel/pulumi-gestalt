#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterConfigPreemptibleWorkerConfigInstanceFlexibilityPolicyInstanceSelectionList {
    /// Full machine-type names, e.g. `"n1-standard-16"`.
    #[builder(into)]
    #[serde(rename = "machineTypes")]
    pub r#machine_types: Option<Vec<String>>,
    /// Preference of this instance selection. A lower number means higher preference. Dataproc will first try to create a VM based on the machine-type with priority rank and fallback to next rank based on availability. Machine types and instance selections with the same priority have the same preference.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "rank")]
    pub r#rank: Option<i32>,
}
