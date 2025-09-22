#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareClusterStatus {
    /// (Output)
    /// ResourceConditions provide a standard mechanism for higher-level status reporting from user cluster controller.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Option<Vec<super::super::types::gkeonprem::VMwareClusterStatusCondition>>,
    /// (Output)
    /// Human-friendly representation of the error message from the user cluster
    /// controller. The error message can be temporary as the user cluster
    /// controller creates a cluster or node pool. If the error message persists
    /// for a longer period of time, it can be used to surface error message to
    /// indicate real problems requiring user intervention.
    #[builder(into)]
    #[serde(rename = "errorMessage")]
    pub r#error_message: Option<String>,
}
