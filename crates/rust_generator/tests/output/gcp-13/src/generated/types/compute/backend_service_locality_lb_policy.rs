#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackendServiceLocalityLbPolicy {
    /// The configuration for a custom policy implemented by the user and
    /// deployed with the client.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customPolicy")]
    pub r#custom_policy: Box<Option<super::super::types::compute::BackendServiceLocalityLbPolicyCustomPolicy>>,
    /// The configuration for a built-in load balancing policy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policy")]
    pub r#policy: Box<Option<super::super::types::compute::BackendServiceLocalityLbPolicyPolicy>>,
}
