#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkAttachedDataNetworkNetworkAddressPortTranslation {
    /// Pinhole timeout for ICMP pinholes in seconds. Must between `1` to `180`, Default to `180`.
    #[builder(into)]
    #[serde(rename = "icmpPinholeTimeoutInSeconds")]
    pub r#icmp_pinhole_timeout_in_seconds: Option<i32>,
    /// Maximum number of UDP and TCP pinholes that can be open simultaneously on the core interface. For 5G networks, this is the N6 interface. For 4G networks, this is the SGi interface. Must be between 1 and 65536.
    #[builder(into)]
    #[serde(rename = "pinholeMaximumNumber")]
    pub r#pinhole_maximum_number: Option<i32>,
    /// A `port_range` block as defined below.
    #[builder(into)]
    #[serde(rename = "portRange")]
    pub r#port_range: Option<Box<super::super::types::mobile::NetworkAttachedDataNetworkNetworkAddressPortTranslationPortRange>>,
    /// Pinhole timeout for TCP pinholes in seconds. Must between `1` to `180`, Default to `180`.
    #[builder(into)]
    #[serde(rename = "tcpPinholeTimeoutInSeconds")]
    pub r#tcp_pinhole_timeout_in_seconds: Option<i32>,
    /// Minimum time in seconds that will pass before a TCP port that was used by a closed pinhole can be reused. Defaults to `120`.
    #[builder(into)]
    #[serde(rename = "tcpPortReuseMinimumHoldTimeInSeconds")]
    pub r#tcp_port_reuse_minimum_hold_time_in_seconds: Option<i32>,
    /// Pinhole timeout for UDP pinholes in seconds. Must between `1` to `180`, Default to `180`.
    #[builder(into)]
    #[serde(rename = "udpPinholeTimeoutInSeconds")]
    pub r#udp_pinhole_timeout_in_seconds: Option<i32>,
    /// Minimum time in seconds that will pass before a UDP port that was used by a closed pinhole can be reused. Defaults to `60`.
    #[builder(into)]
    #[serde(rename = "udpPortReuseMinimumHoldTimeInSeconds")]
    pub r#udp_port_reuse_minimum_hold_time_in_seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkAttachedDataNetworkNetworkAddressPortTranslation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "icmp_pinhole_timeout_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#icmp_pinhole_timeout_in_seconds,
                )
                .await,
            );
            map.insert(
                "pinhole_maximum_number".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pinhole_maximum_number,
                )
                .await,
            );
            map.insert(
                "port_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port_range,
                )
                .await,
            );
            map.insert(
                "tcp_pinhole_timeout_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tcp_pinhole_timeout_in_seconds,
                )
                .await,
            );
            map.insert(
                "tcp_port_reuse_minimum_hold_time_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tcp_port_reuse_minimum_hold_time_in_seconds,
                )
                .await,
            );
            map.insert(
                "udp_pinhole_timeout_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#udp_pinhole_timeout_in_seconds,
                )
                .await,
            );
            map.insert(
                "udp_port_reuse_minimum_hold_time_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#udp_port_reuse_minimum_hold_time_in_seconds,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkAttachedDataNetworkNetworkAddressPortTranslation {
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
                    r#icmp_pinhole_timeout_in_seconds: {
                        let field_value = match fields_map.get("icmp_pinhole_timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'icmp_pinhole_timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pinhole_maximum_number: {
                        let field_value = match fields_map.get("pinhole_maximum_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'pinhole_maximum_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_range: {
                        let field_value = match fields_map.get("port_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'port_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tcp_pinhole_timeout_in_seconds: {
                        let field_value = match fields_map.get("tcp_pinhole_timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcp_pinhole_timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tcp_port_reuse_minimum_hold_time_in_seconds: {
                        let field_value = match fields_map.get("tcp_port_reuse_minimum_hold_time_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcp_port_reuse_minimum_hold_time_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#udp_pinhole_timeout_in_seconds: {
                        let field_value = match fields_map.get("udp_pinhole_timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'udp_pinhole_timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#udp_port_reuse_minimum_hold_time_in_seconds: {
                        let field_value = match fields_map.get("udp_port_reuse_minimum_hold_time_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'udp_port_reuse_minimum_hold_time_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
