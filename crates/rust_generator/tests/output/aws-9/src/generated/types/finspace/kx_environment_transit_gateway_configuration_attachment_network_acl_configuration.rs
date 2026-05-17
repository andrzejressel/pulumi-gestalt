#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfiguration {
    /// The IPv4 network range to allow or deny, in CIDR notation. The specified CIDR block is modified to its canonical form. For example, `100.68.0.18/18` will be converted to `100.68.0.0/18`.
    #[builder(into)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: String,
    /// Defines the ICMP protocol that consists of the ICMP type and code. Defined below.
    #[builder(into)]
    #[serde(rename = "icmpTypeCode")]
    pub r#icmp_type_code: Option<Box<super::super::types::finspace::KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfigurationIcmpTypeCode>>,
    /// Range of ports the rule applies to. Defined below.
    #[builder(into)]
    #[serde(rename = "portRange")]
    pub r#port_range: Option<Box<super::super::types::finspace::KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfigurationPortRange>>,
    /// Protocol number. A value of `1` means all the protocols.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// Indicates whether to `allow` or `deny` the traffic that matches the rule.
    #[builder(into)]
    #[serde(rename = "ruleAction")]
    pub r#rule_action: String,
    /// Rule number for the entry. All the network ACL entries are processed in ascending order by rule number.
    #[builder(into)]
    #[serde(rename = "ruleNumber")]
    pub r#rule_number: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "cidr_block",
                    &self.r#cidr_block,
                ),
                to_pulumi_object_field(
                    "icmp_type_code",
                    &self.r#icmp_type_code,
                ),
                to_pulumi_object_field(
                    "port_range",
                    &self.r#port_range,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "rule_action",
                    &self.r#rule_action,
                ),
                to_pulumi_object_field(
                    "rule_number",
                    &self.r#rule_number,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfiguration {
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
                    r#cidr_block: {
                        let field_value = match fields_map.get("cidr_block") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidr_block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#icmp_type_code: {
                        let field_value = match fields_map.get("icmp_type_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'icmp_type_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_action: {
                        let field_value = match fields_map.get("rule_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_number: {
                        let field_value = match fields_map.get("rule_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
