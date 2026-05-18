#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuleGroupRuleGroupRulesSourceStatefulRuleHeader {
    /// The destination IP address or address range to inspect for, in CIDR notation. To match with any address, specify `ANY`.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: String,
    /// The destination port to inspect for. To match with any address, specify `ANY`.
    #[builder(into)]
    #[serde(rename = "destinationPort")]
    pub r#destination_port: String,
    /// The direction of traffic flow to inspect. Valid values: `ANY` or `FORWARD`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: String,
    /// The protocol to inspect. Valid values: `IP`, `TCP`, `UDP`, `ICMP`, `HTTP`, `FTP`, `TLS`, `SMB`, `DNS`, `DCERPC`, `SSH`, `SMTP`, `IMAP`, `MSN`, `KRB5`, `IKEV2`, `TFTP`, `NTP`, `DHCP`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// The source IP address or address range for, in CIDR notation. To match with any address, specify `ANY`.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: String,
    /// The source port to inspect for. To match with any address, specify `ANY`.
    #[builder(into)]
    #[serde(rename = "sourcePort")]
    pub r#source_port: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuleGroupRuleGroupRulesSourceStatefulRuleHeader {
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
                    "destination",
                    &self.r#destination,
                ),
                to_pulumi_object_field(
                    "destination_port",
                    &self.r#destination_port,
                ),
                to_pulumi_object_field(
                    "direction",
                    &self.r#direction,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "source",
                    &self.r#source,
                ),
                to_pulumi_object_field(
                    "source_port",
                    &self.r#source_port,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuleGroupRuleGroupRulesSourceStatefulRuleHeader {
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
                    r#destination: {
                        let field_value = match fields_map.get("destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_port: {
                        let field_value = match fields_map.get("destination_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#direction: {
                        let field_value = match fields_map.get("direction") {
                            Some(value) => value,
                            None => bail!("Missing field 'direction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#source: {
                        let field_value = match fields_map.get("source") {
                            Some(value) => value,
                            None => bail!("Missing field 'source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_port: {
                        let field_value = match fields_map.get("source_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
