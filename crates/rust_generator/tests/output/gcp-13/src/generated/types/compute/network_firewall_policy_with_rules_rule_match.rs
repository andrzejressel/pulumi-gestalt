#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkFirewallPolicyWithRulesRuleMatch {
    /// Address groups which should be matched against the traffic destination.
    /// Maximum number of destination address groups is 10.
    #[builder(into)]
    #[serde(rename = "destAddressGroups")]
    pub r#dest_address_groups: Option<Vec<String>>,
    /// Fully Qualified Domain Name (FQDN) which should be matched against
    /// traffic destination. Maximum number of destination fqdn allowed is 100.
    #[builder(into)]
    #[serde(rename = "destFqdns")]
    pub r#dest_fqdns: Option<Vec<String>>,
    /// Destination IP address range in CIDR format. Required for
    /// EGRESS rules.
    #[builder(into)]
    #[serde(rename = "destIpRanges")]
    pub r#dest_ip_ranges: Option<Vec<String>>,
    /// Region codes whose IP addresses will be used to match for destination
    /// of traffic. Should be specified as 2 letter country code defined as per
    /// ISO 3166 alpha-2 country codes. ex."US"
    /// Maximum number of destination region codes allowed is 5000.
    #[builder(into)]
    #[serde(rename = "destRegionCodes")]
    pub r#dest_region_codes: Option<Vec<String>>,
    /// Names of Network Threat Intelligence lists.
    /// The IPs in these lists will be matched against traffic destination.
    #[builder(into)]
    #[serde(rename = "destThreatIntelligences")]
    pub r#dest_threat_intelligences: Option<Vec<String>>,
    /// Pairs of IP protocols and ports that the rule should match.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "layer4Configs")]
    pub r#layer_4_configs: Vec<super::super::types::compute::NetworkFirewallPolicyWithRulesRuleMatchLayer4Config>,
    /// Address groups which should be matched against the traffic source.
    /// Maximum number of source address groups is 10.
    #[builder(into)]
    #[serde(rename = "srcAddressGroups")]
    pub r#src_address_groups: Option<Vec<String>>,
    /// Fully Qualified Domain Name (FQDN) which should be matched against
    /// traffic source. Maximum number of source fqdn allowed is 100.
    #[builder(into)]
    #[serde(rename = "srcFqdns")]
    pub r#src_fqdns: Option<Vec<String>>,
    /// Source IP address range in CIDR format. Required for
    /// INGRESS rules.
    #[builder(into)]
    #[serde(rename = "srcIpRanges")]
    pub r#src_ip_ranges: Option<Vec<String>>,
    /// Region codes whose IP addresses will be used to match for source
    /// of traffic. Should be specified as 2 letter country code defined as per
    /// ISO 3166 alpha-2 country codes. ex."US"
    /// Maximum number of source region codes allowed is 5000.
    #[builder(into)]
    #[serde(rename = "srcRegionCodes")]
    pub r#src_region_codes: Option<Vec<String>>,
    /// List of secure tag values, which should be matched at the source
    /// of the traffic.
    /// For INGRESS rule, if all the <code>srcSecureTag</code> are INEFFECTIVE,
    /// and there is no <code>srcIpRange</code>, this rule will be ignored.
    /// Maximum number of source tag values allowed is 256.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_layer4_config"></a>The `layer4_config` block supports:
    #[builder(into)]
    #[serde(rename = "srcSecureTags")]
    pub r#src_secure_tags: Option<Vec<super::super::types::compute::NetworkFirewallPolicyWithRulesRuleMatchSrcSecureTag>>,
    /// Names of Network Threat Intelligence lists.
    /// The IPs in these lists will be matched against traffic source.
    #[builder(into)]
    #[serde(rename = "srcThreatIntelligences")]
    pub r#src_threat_intelligences: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkFirewallPolicyWithRulesRuleMatch {
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
                "dest_address_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dest_address_groups,
                )
                .await,
            );
            map.insert(
                "dest_fqdns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dest_fqdns,
                )
                .await,
            );
            map.insert(
                "dest_ip_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dest_ip_ranges,
                )
                .await,
            );
            map.insert(
                "dest_region_codes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dest_region_codes,
                )
                .await,
            );
            map.insert(
                "dest_threat_intelligences".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dest_threat_intelligences,
                )
                .await,
            );
            map.insert(
                "layer_4_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#layer_4_configs,
                )
                .await,
            );
            map.insert(
                "src_address_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#src_address_groups,
                )
                .await,
            );
            map.insert(
                "src_fqdns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#src_fqdns,
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
                "src_region_codes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#src_region_codes,
                )
                .await,
            );
            map.insert(
                "src_secure_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#src_secure_tags,
                )
                .await,
            );
            map.insert(
                "src_threat_intelligences".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#src_threat_intelligences,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkFirewallPolicyWithRulesRuleMatch {
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
                    r#dest_address_groups: {
                        let field_value = match fields_map.get("dest_address_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'dest_address_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dest_fqdns: {
                        let field_value = match fields_map.get("dest_fqdns") {
                            Some(value) => value,
                            None => bail!("Missing field 'dest_fqdns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dest_ip_ranges: {
                        let field_value = match fields_map.get("dest_ip_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'dest_ip_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dest_region_codes: {
                        let field_value = match fields_map.get("dest_region_codes") {
                            Some(value) => value,
                            None => bail!("Missing field 'dest_region_codes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dest_threat_intelligences: {
                        let field_value = match fields_map.get("dest_threat_intelligences") {
                            Some(value) => value,
                            None => bail!("Missing field 'dest_threat_intelligences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#layer_4_configs: {
                        let field_value = match fields_map.get("layer_4_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'layer_4_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#src_address_groups: {
                        let field_value = match fields_map.get("src_address_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'src_address_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#src_fqdns: {
                        let field_value = match fields_map.get("src_fqdns") {
                            Some(value) => value,
                            None => bail!("Missing field 'src_fqdns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#src_region_codes: {
                        let field_value = match fields_map.get("src_region_codes") {
                            Some(value) => value,
                            None => bail!("Missing field 'src_region_codes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#src_secure_tags: {
                        let field_value = match fields_map.get("src_secure_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'src_secure_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#src_threat_intelligences: {
                        let field_value = match fields_map.get("src_threat_intelligences") {
                            Some(value) => value,
                            None => bail!("Missing field 'src_threat_intelligences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
