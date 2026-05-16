#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetEc2InboundPermission {
    /// Starting value for a range of allowed port numbers.
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: i32,
    /// Range of allowed IP addresses expressed in CIDR notationE.g., `000.000.000.000/[subnet mask]` or `0.0.0.0/[subnet mask]`.
    #[builder(into)]
    #[serde(rename = "ipRange")]
    pub r#ip_range: String,
    /// Network communication protocol used by the fleetE.g., `TCP` or `UDP`
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    /// Ending value for a range of allowed port numbers. Port numbers are end-inclusive. This value must be higher than `from_port`.
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetEc2InboundPermission {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("from_port".to_string(), self.r#from_port.to_pulumi_value().await);
            map.insert("ip_range".to_string(), self.r#ip_range.to_pulumi_value().await);
            map.insert("protocol".to_string(), self.r#protocol.to_pulumi_value().await);
            map.insert("to_port".to_string(), self.r#to_port.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetEc2InboundPermission {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#from_port: {
                        let field_value = match fields_map.get("from_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'from_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ip_range: {
                        let field_value = match fields_map.get("ip_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#to_port: {
                        let field_value = match fields_map.get("to_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'to_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
