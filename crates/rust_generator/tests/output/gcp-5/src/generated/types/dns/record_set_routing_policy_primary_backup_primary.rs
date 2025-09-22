#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecordSetRoutingPolicyPrimaryBackupPrimary {
    /// The list of internal load balancers to health check.
    #[builder(into)]
    #[serde(rename = "internalLoadBalancers")]
    pub r#internal_load_balancers: Vec<super::super::types::dns::RecordSetRoutingPolicyPrimaryBackupPrimaryInternalLoadBalancer>,
}
