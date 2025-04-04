#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkSimPolicySliceDataNetwork {
    /// Allowed session types in addition to the default session type.
    #[builder(into)]
    #[serde(rename = "additionalAllowedSessionTypes")]
    pub r#additional_allowed_session_types: Box<Vec<String>>,
    /// Default QoS Flow allocation and retention priority (ARP) level. Flows with higher priority preempt flows with lower priority, if the settings of `preemption_capability` and `preemption_vulnerability` allow it. 1 is the highest level of priority. If this field is not specified then `qos_indicator` is used to derive the ARP value. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters.
    #[builder(into)]
    #[serde(rename = "allocationAndRetentionPriorityLevel")]
    pub r#allocation_and_retention_priority_level: Box<i32>,
    /// An array of IDs of services that can be used as part of this SIM policy.
    #[builder(into)]
    #[serde(rename = "allowedServicesIds")]
    pub r#allowed_services_ids: Box<Vec<String>>,
    /// The ID of Mobile Network Data Network which these settings apply to.
    #[builder(into)]
    #[serde(rename = "dataNetworkId")]
    pub r#data_network_id: Box<String>,
    /// The default PDU session type, which is used if the UE does not request a specific session type.
    #[builder(into)]
    #[serde(rename = "defaultSessionType")]
    pub r#default_session_type: Box<String>,
    /// The maximum number of downlink packets to buffer at the user plane for High Latency Communication - Extended Buffering.
    #[builder(into)]
    #[serde(rename = "maxBufferedPackets")]
    pub r#max_buffered_packets: Box<i32>,
    /// The Preemption Capability of a QoS Flow controls whether it can preempt another QoS Flow with a lower priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters.
    #[builder(into)]
    #[serde(rename = "preemptionCapability")]
    pub r#preemption_capability: Box<String>,
    /// The Preemption Vulnerability of a QoS Flow controls whether it can be preempted by QoS Flow with a higher priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters.
    #[builder(into)]
    #[serde(rename = "preemptionVulnerability")]
    pub r#preemption_vulnerability: Box<String>,
    /// The QoS Indicator (5QI for 5G network /QCI for 4G net work) value identifies a set of QoS characteristics that control QoS forwarding treatment for QoS flows or EPS bearers.
    #[builder(into)]
    #[serde(rename = "qosIndicator")]
    pub r#qos_indicator: Box<i32>,
    /// A `session_aggregate_maximum_bit_rate` block as defined below.
    #[builder(into)]
    #[serde(rename = "sessionAggregateMaximumBitRates")]
    pub r#session_aggregate_maximum_bit_rates: Box<Vec<super::super::types::mobile::GetNetworkSimPolicySliceDataNetworkSessionAggregateMaximumBitRate>>,
}
