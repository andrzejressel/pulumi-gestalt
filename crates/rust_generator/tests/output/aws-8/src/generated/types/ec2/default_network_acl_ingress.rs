#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DefaultNetworkAclIngress {
    /// The action to take.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// The CIDR block to match. This must be a valid network mask.
    #[builder(into)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Option<String>,
    /// The from port to match.
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: i32,
    /// The ICMP type code to be used. Default 0.
    #[builder(into)]
    #[serde(rename = "icmpCode")]
    pub r#icmp_code: Option<i32>,
    /// The ICMP type to be used. Default 0.
    #[builder(into)]
    #[serde(rename = "icmpType")]
    pub r#icmp_type: Option<i32>,
    /// The IPv6 CIDR block.
    /// 
    /// > For more information on ICMP types and codes, see [Internet Control Message Protocol (ICMP) Parameters](https://www.iana.org/assignments/icmp-parameters/icmp-parameters.xhtml).
    #[builder(into)]
    #[serde(rename = "ipv6CidrBlock")]
    pub r#ipv_6_cidr_block: Option<String>,
    /// The protocol to match. If using the -1 'all' protocol, you must specify a from and to port of 0.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// The rule number. Used for ordering.
    #[builder(into)]
    #[serde(rename = "ruleNo")]
    pub r#rule_no: i32,
    /// The to port to match.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DefaultNetworkAclIngress {
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
                "action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#action,
                )
                .await,
            );
            map.insert(
                "cidr_block".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cidr_block,
                )
                .await,
            );
            map.insert(
                "from_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#from_port,
                )
                .await,
            );
            map.insert(
                "icmp_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#icmp_code,
                )
                .await,
            );
            map.insert(
                "icmp_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#icmp_type,
                )
                .await,
            );
            map.insert(
                "ipv_6_cidr_block".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipv_6_cidr_block,
                )
                .await,
            );
            map.insert(
                "protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocol,
                )
                .await,
            );
            map.insert(
                "rule_no".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_no,
                )
                .await,
            );
            map.insert(
                "to_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#to_port,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DefaultNetworkAclIngress {
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
                    r#action: {
                        let field_value = match fields_map.get("action") {
                            Some(value) => value,
                            None => bail!("Missing field 'action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cidr_block: {
                        let field_value = match fields_map.get("cidr_block") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidr_block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from_port: {
                        let field_value = match fields_map.get("from_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'from_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#icmp_code: {
                        let field_value = match fields_map.get("icmp_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'icmp_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#icmp_type: {
                        let field_value = match fields_map.get("icmp_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'icmp_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6_cidr_block: {
                        let field_value = match fields_map.get("ipv_6_cidr_block") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6_cidr_block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_no: {
                        let field_value = match fields_map.get("rule_no") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_no' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#to_port: {
                        let field_value = match fields_map.get("to_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'to_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
