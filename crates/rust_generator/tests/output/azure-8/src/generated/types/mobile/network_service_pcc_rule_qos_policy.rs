#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkServicePccRuleQosPolicy {
    /// QoS Flow allocation and retention priority (ARP) level. Flows with higher priority preempt flows with lower priority, if the settings of `preemption_capability` and `preemption_vulnerability` allow it. 1 is the highest level of priority. If this field is not specified then `qos_indicator` is used to derive the ARP value. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters.
    #[builder(into)]
    #[serde(rename = "allocationAndRetentionPriorityLevel")]
    pub r#allocation_and_retention_priority_level: Option<i32>,
    /// A `guaranteed_bit_rate` block as defined below. The Guaranteed Bit Rate (GBR) for all service data flows that use this PCC Rule. If it's not specified, there will be no GBR set for the PCC Rule that uses this QoS definition.
    #[builder(into)]
    #[serde(rename = "guaranteedBitRate")]
    pub r#guaranteed_bit_rate: Option<Box<super::super::types::mobile::NetworkServicePccRuleQosPolicyGuaranteedBitRate>>,
    /// A `maximum_bit_rate` block as defined below. The Maximum Bit Rate (MBR) for all service data flows that use this PCC Rule or Service.
    #[builder(into)]
    #[serde(rename = "maximumBitRate")]
    pub r#maximum_bit_rate: Box<super::super::types::mobile::NetworkServicePccRuleQosPolicyMaximumBitRate>,
    /// The Preemption Capability of a QoS Flow controls whether it can preempt another QoS Flow with a lower priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters. Possible values are `NotPreempt` and `MayPreempt`, Defaults to `NotPreempt`.
    #[builder(into)]
    #[serde(rename = "preemptionCapability")]
    pub r#preemption_capability: Option<String>,
    /// The Preemption Vulnerability of a QoS Flow controls whether it can be preempted by QoS Flow with a higher priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters. Possible values are `NotPreemptable` and `Preemptable`. Defaults to `Preemptable`.
    #[builder(into)]
    #[serde(rename = "preemptionVulnerability")]
    pub r#preemption_vulnerability: Option<String>,
    /// The QoS Indicator (5QI for 5G network /QCI for 4G net work) value identifies a set of QoS characteristics that control QoS forwarding treatment for QoS flows or EPS bearers. Recommended values: 5-9; 69-70; 79-80. Must be between `1` and `127`.
    #[builder(into)]
    #[serde(rename = "qosIndicator")]
    pub r#qos_indicator: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkServicePccRuleQosPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "allocation_and_retention_priority_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allocation_and_retention_priority_level,
                )
                .await,
            );
            map.insert(
                "guaranteed_bit_rate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#guaranteed_bit_rate,
                )
                .await,
            );
            map.insert(
                "maximum_bit_rate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_bit_rate,
                )
                .await,
            );
            map.insert(
                "preemption_capability".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preemption_capability,
                )
                .await,
            );
            map.insert(
                "preemption_vulnerability".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preemption_vulnerability,
                )
                .await,
            );
            map.insert(
                "qos_indicator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#qos_indicator,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkServicePccRuleQosPolicy {
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
                    r#allocation_and_retention_priority_level: {
                        let field_value = match fields_map.get("allocation_and_retention_priority_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocation_and_retention_priority_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#guaranteed_bit_rate: {
                        let field_value = match fields_map.get("guaranteed_bit_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'guaranteed_bit_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_bit_rate: {
                        let field_value = match fields_map.get("maximum_bit_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_bit_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
