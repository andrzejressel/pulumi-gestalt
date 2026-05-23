#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkAttachedDataNetworkNetworkAddressPortTranslation {
    #[builder(into)]
    #[serde(rename = "icmpPinholeTimeoutInSeconds")]
    pub r#icmp_pinhole_timeout_in_seconds: i32,
    #[builder(into)]
    #[serde(rename = "pinholeMaximumNumber")]
    pub r#pinhole_maximum_number: Option<i32>,
    /// A `port_range` block as defined below.
    #[builder(into)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Vec<super::super::types::mobile::GetNetworkAttachedDataNetworkNetworkAddressPortTranslationPortRange>,
    #[builder(into)]
    #[serde(rename = "tcpPinholeTimeoutInSeconds")]
    pub r#tcp_pinhole_timeout_in_seconds: i32,
    /// Minimum time in seconds that will pass before a TCP port that was used by a closed pinhole can be reused.
    #[builder(into)]
    #[serde(rename = "tcpPortReuseMinimumHoldTimeInSeconds")]
    pub r#tcp_port_reuse_minimum_hold_time_in_seconds: i32,
    #[builder(into)]
    #[serde(rename = "udpPinholeTimeoutInSeconds")]
    pub r#udp_pinhole_timeout_in_seconds: i32,
    /// Minimum time in seconds that will pass before a UDP port that was used by a closed pinhole can be reused.
    #[builder(into)]
    #[serde(rename = "udpPortReuseMinimumHoldTimeInSeconds")]
    pub r#udp_port_reuse_minimum_hold_time_in_seconds: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkAttachedDataNetworkNetworkAddressPortTranslation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "icmpPinholeTimeoutInSeconds",
                    &self.r#icmp_pinhole_timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "pinholeMaximumNumber",
                    &self.r#pinhole_maximum_number,
                ),
                to_pulumi_object_field(
                    "portRanges",
                    &self.r#port_ranges,
                ),
                to_pulumi_object_field(
                    "tcpPinholeTimeoutInSeconds",
                    &self.r#tcp_pinhole_timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "tcpPortReuseMinimumHoldTimeInSeconds",
                    &self.r#tcp_port_reuse_minimum_hold_time_in_seconds,
                ),
                to_pulumi_object_field(
                    "udpPinholeTimeoutInSeconds",
                    &self.r#udp_pinhole_timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "udpPortReuseMinimumHoldTimeInSeconds",
                    &self.r#udp_port_reuse_minimum_hold_time_in_seconds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkAttachedDataNetworkNetworkAddressPortTranslation {
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
                        let field_value = match fields_map.get("icmpPinholeTimeoutInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'icmpPinholeTimeoutInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pinhole_maximum_number: {
                        let field_value = match fields_map.get("pinholeMaximumNumber") {
                            Some(value) => value,
                            None => bail!("Missing field 'pinholeMaximumNumber' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_ranges: {
                        let field_value = match fields_map.get("portRanges") {
                            Some(value) => value,
                            None => bail!("Missing field 'portRanges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tcp_pinhole_timeout_in_seconds: {
                        let field_value = match fields_map.get("tcpPinholeTimeoutInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcpPinholeTimeoutInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tcp_port_reuse_minimum_hold_time_in_seconds: {
                        let field_value = match fields_map.get("tcpPortReuseMinimumHoldTimeInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcpPortReuseMinimumHoldTimeInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#udp_pinhole_timeout_in_seconds: {
                        let field_value = match fields_map.get("udpPinholeTimeoutInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'udpPinholeTimeoutInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#udp_port_reuse_minimum_hold_time_in_seconds: {
                        let field_value = match fields_map.get("udpPortReuseMinimumHoldTimeInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'udpPortReuseMinimumHoldTimeInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
