#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PerInstanceConfigPreservedState {
    /// Stateful disks for the instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "disks")]
    pub r#disks: Option<Vec<super::super::types::compute::PerInstanceConfigPreservedStateDisk>>,
    /// Preserved external IPs defined for this instance. This map is keyed with the name of the network interface.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "externalIps")]
    pub r#external_ips: Option<Vec<super::super::types::compute::PerInstanceConfigPreservedStateExternalIp>>,
    /// Preserved internal IPs defined for this instance. This map is keyed with the name of the network interface.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "internalIps")]
    pub r#internal_ips: Option<Vec<super::super::types::compute::PerInstanceConfigPreservedStateInternalIp>>,
    /// Preserved metadata defined for this instance. This is a list of key->value pairs.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Option<std::collections::HashMap<String, String>>,
}
