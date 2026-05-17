#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkSimPolicySliceDataNetwork {
    /// Allowed session types in addition to the default session type. Must not duplicate the default session type. Possible values are `IPv4` and `IPv6`.
    #[builder(into)]
    #[serde(rename = "additionalAllowedSessionTypes")]
    pub r#additional_allowed_session_types: Option<Vec<String>>,
    /// Default QoS Flow allocation and retention priority (ARP) level. Flows with higher priority preempt flows with lower priority, if the settings of `preemption_capability` and `preemption_vulnerability` allow it. `1` is the highest level of priority. If this field is not specified then `qos_indicator` is used to derive the ARP value. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters.
    #[builder(into)]
    #[serde(rename = "allocationAndRetentionPriorityLevel")]
    pub r#allocation_and_retention_priority_level: Option<i32>,
    /// An array of IDs of services that can be used as part of this SIM policy. The array must not contain duplicate items and must contain at least one item.
    #[builder(into)]
    #[serde(rename = "allowedServicesIds")]
    pub r#allowed_services_ids: Vec<String>,
    /// The ID of Mobile Network Data Network which these settings apply to.
    #[builder(into)]
    #[serde(rename = "dataNetworkId")]
    pub r#data_network_id: String,
    /// The default PDU session type, which is used if the user equipment does not request a specific session type. Possible values are `IPv4` and `IPv6`. Defaults to `IPv4`.
    #[builder(into)]
    #[serde(rename = "defaultSessionType")]
    pub r#default_session_type: Option<String>,
    /// The maximum number of downlink packets to buffer at the user plane for High Latency Communication - Extended Buffering. Defaults to `10`, Must be at least `0`, See 3GPP TS29.272 v15.10.0 section 7.3.188 for a full description. This maximum is not guaranteed because there is a internal limit on buffered packets across all PDU sessions.
    #[builder(into)]
    #[serde(rename = "maxBufferedPackets")]
    pub r#max_buffered_packets: Option<i32>,
    /// The Preemption Capability of a QoS Flow, it controls whether it can preempt another QoS Flow with a lower priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters. Possible values are `NotPreempt` and `MayPreempt`, Defaults to `NotPreempt`.
    #[builder(into)]
    #[serde(rename = "preemptionCapability")]
    pub r#preemption_capability: Option<String>,
    /// The Preemption Vulnerability of a QoS Flow, it controls whether it can be preempted by QoS Flow with a higher priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters. Possible values are `NotPreemptable` and `Preemptable`. Defaults to `NotPreemptable`.
    #[builder(into)]
    #[serde(rename = "preemptionVulnerability")]
    pub r#preemption_vulnerability: Option<String>,
    /// The QoS Indicator (5QI for 5G network /QCI for 4G net work) value identifies a set of QoS characteristics, it controls QoS forwarding treatment for QoS flows or EPS bearers. Recommended values: 5-9; 69-70; 79-80. Must be between `1` and `127`.
    #[builder(into)]
    #[serde(rename = "qosIndicator")]
    pub r#qos_indicator: i32,
    /// A `session_aggregate_maximum_bit_rate` block as defined below.
    #[builder(into)]
    #[serde(rename = "sessionAggregateMaximumBitRate")]
    pub r#session_aggregate_maximum_bit_rate: Box<super::super::types::mobile::NetworkSimPolicySliceDataNetworkSessionAggregateMaximumBitRate>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkSimPolicySliceDataNetwork {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "additional_allowed_session_types",
                    &self.r#additional_allowed_session_types,
                ),
                to_pulumi_object_field(
                    "allocation_and_retention_priority_level",
                    &self.r#allocation_and_retention_priority_level,
                ),
                to_pulumi_object_field(
                    "allowed_services_ids",
                    &self.r#allowed_services_ids,
                ),
                to_pulumi_object_field(
                    "data_network_id",
                    &self.r#data_network_id,
                ),
                to_pulumi_object_field(
                    "default_session_type",
                    &self.r#default_session_type,
                ),
                to_pulumi_object_field(
                    "max_buffered_packets",
                    &self.r#max_buffered_packets,
                ),
                to_pulumi_object_field(
                    "preemption_capability",
                    &self.r#preemption_capability,
                ),
                to_pulumi_object_field(
                    "preemption_vulnerability",
                    &self.r#preemption_vulnerability,
                ),
                to_pulumi_object_field(
                    "qos_indicator",
                    &self.r#qos_indicator,
                ),
                to_pulumi_object_field(
                    "session_aggregate_maximum_bit_rate",
                    &self.r#session_aggregate_maximum_bit_rate,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkSimPolicySliceDataNetwork {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#additional_allowed_session_types: {
                        let field_value = match fields_map.get("additional_allowed_session_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_allowed_session_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allocation_and_retention_priority_level: {
                        let field_value = match fields_map.get("allocation_and_retention_priority_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocation_and_retention_priority_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_services_ids: {
                        let field_value = match fields_map.get("allowed_services_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_services_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_network_id: {
                        let field_value = match fields_map.get("data_network_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_network_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_session_type: {
                        let field_value = match fields_map.get("default_session_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_session_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_buffered_packets: {
                        let field_value = match fields_map.get("max_buffered_packets") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_buffered_packets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preemption_capability: {
                        let field_value = match fields_map.get("preemption_capability") {
                            Some(value) => value,
                            None => bail!("Missing field 'preemption_capability' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preemption_vulnerability: {
                        let field_value = match fields_map.get("preemption_vulnerability") {
                            Some(value) => value,
                            None => bail!("Missing field 'preemption_vulnerability' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qos_indicator: {
                        let field_value = match fields_map.get("qos_indicator") {
                            Some(value) => value,
                            None => bail!("Missing field 'qos_indicator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_aggregate_maximum_bit_rate: {
                        let field_value = match fields_map.get("session_aggregate_maximum_bit_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_aggregate_maximum_bit_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
