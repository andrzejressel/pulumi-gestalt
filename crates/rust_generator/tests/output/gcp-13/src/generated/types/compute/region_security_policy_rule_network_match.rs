#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionSecurityPolicyRuleNetworkMatch {
    /// Destination IPv4/IPv6 addresses or CIDR prefixes, in standard text format.
    #[builder(into)]
    #[serde(rename = "destIpRanges")]
    pub r#dest_ip_ranges: Option<Vec<String>>,
    /// Destination port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. "80") or range (e.g. "0-1023").
    #[builder(into)]
    #[serde(rename = "destPorts")]
    pub r#dest_ports: Option<Vec<String>>,
    /// IPv4 protocol / IPv6 next header (after extension headers). Each element can be an 8-bit unsigned decimal number (e.g. "6"), range (e.g. "253-254"), or one of the following protocol names: "tcp", "udp", "icmp", "esp", "ah", "ipip", or "sctp".
    #[builder(into)]
    #[serde(rename = "ipProtocols")]
    pub r#ip_protocols: Option<Vec<String>>,
    /// BGP Autonomous System Number associated with the source IP address.
    #[builder(into)]
    #[serde(rename = "srcAsns")]
    pub r#src_asns: Option<Vec<i32>>,
    /// Source IPv4/IPv6 addresses or CIDR prefixes, in standard text format.
    #[builder(into)]
    #[serde(rename = "srcIpRanges")]
    pub r#src_ip_ranges: Option<Vec<String>>,
    /// Source port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. "80") or range (e.g. "0-1023").
    #[builder(into)]
    #[serde(rename = "srcPorts")]
    pub r#src_ports: Option<Vec<String>>,
    /// Two-letter ISO 3166-1 alpha-2 country code associated with the source IP address.
    #[builder(into)]
    #[serde(rename = "srcRegionCodes")]
    pub r#src_region_codes: Option<Vec<String>>,
    /// User-defined fields. Each element names a defined field and lists the matching values for that field.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "userDefinedFields")]
    pub r#user_defined_fields: Option<Vec<super::super::types::compute::RegionSecurityPolicyRuleNetworkMatchUserDefinedField>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionSecurityPolicyRuleNetworkMatch {
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
                "dest_ip_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dest_ip_ranges,
                )
                .await,
            );
            map.insert(
                "dest_ports".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dest_ports,
                )
                .await,
            );
            map.insert(
                "ip_protocols".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_protocols,
                )
                .await,
            );
            map.insert(
                "src_asns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#src_asns,
                )
                .await,
            );
            map.insert(
                "src_ip_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#src_ip_ranges,
                )
                .await,
            );
            map.insert(
                "src_ports".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#src_ports,
                )
                .await,
            );
            map.insert(
                "src_region_codes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#src_region_codes,
                )
                .await,
            );
            map.insert(
                "user_defined_fields".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_defined_fields,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionSecurityPolicyRuleNetworkMatch {
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
                    r#dest_ip_ranges: {
                        let field_value = match fields_map.get("dest_ip_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'dest_ip_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dest_ports: {
                        let field_value = match fields_map.get("dest_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'dest_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_protocols: {
                        let field_value = match fields_map.get("ip_protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#src_asns: {
                        let field_value = match fields_map.get("src_asns") {
                            Some(value) => value,
                            None => bail!("Missing field 'src_asns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#src_ip_ranges: {
                        let field_value = match fields_map.get("src_ip_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'src_ip_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#src_ports: {
                        let field_value = match fields_map.get("src_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'src_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#src_region_codes: {
                        let field_value = match fields_map.get("src_region_codes") {
                            Some(value) => value,
                            None => bail!("Missing field 'src_region_codes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_defined_fields: {
                        let field_value = match fields_map.get("user_defined_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_defined_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
