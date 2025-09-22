#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkServicePccRuleQosPolicy {
    /// QoS Flow allocation and retention priority (ARP) level.
    #[builder(into)]
    #[serde(rename = "allocationAndRetentionPriorityLevel")]
    pub r#allocation_and_retention_priority_level: i32,
    /// A `guaranteed_bit_rate` block as defined below. The Guaranteed Bit Rate (GBR) for all service data flows that use this PCC Rule.
    #[builder(into)]
    #[serde(rename = "guaranteedBitRates")]
    pub r#guaranteed_bit_rates: Vec<super::super::types::mobile::GetNetworkServicePccRuleQosPolicyGuaranteedBitRate>,
    /// A `maximum_bit_rate` block as defined below. The Maximum Bit Rate (MBR) for all service data flows that use this PCC Rule or Service.
    #[builder(into)]
    #[serde(rename = "maximumBitRates")]
    pub r#maximum_bit_rates: Vec<super::super::types::mobile::GetNetworkServicePccRuleQosPolicyMaximumBitRate>,
    /// The Preemption Capability of a QoS Flow controls whether it can preempt another QoS Flow with a lower priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters.
    #[builder(into)]
    #[serde(rename = "preemptionCapability")]
    pub r#preemption_capability: String,
    /// The Preemption Vulnerability of a QoS Flow controls whether it can be preempted by QoS Flow with a higher priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters.
    #[builder(into)]
    #[serde(rename = "preemptionVulnerability")]
    pub r#preemption_vulnerability: String,
    /// The QoS Indicator (5QI for 5G network /QCI for 4G net work) value identifies a set of QoS characteristics that control QoS forwarding treatment for QoS flows or EPS bearers.
    #[builder(into)]
    #[serde(rename = "qosIndicator")]
    pub r#qos_indicator: i32,
}
