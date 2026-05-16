#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TargetGroupConfig {
    /// The health check configuration.
    #[builder(into)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Option<Box<super::super::types::vpclattice::TargetGroupConfigHealthCheck>>,
    /// The type of IP address used for the target group. Valid values: `IPV4` | `IPV6`.
    #[builder(into)]
    #[serde(rename = "ipAddressType")]
    pub r#ip_address_type: Option<String>,
    /// The version of the event structure that the Lambda function receives. Supported only if `type` is `LAMBDA`. Valid Values are `V1` | `V2`.
    #[builder(into)]
    #[serde(rename = "lambdaEventStructureVersion")]
    pub r#lambda_event_structure_version: Option<String>,
    /// The port on which the targets are listening.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// The protocol to use for routing traffic to the targets. Valid Values are `HTTP` | `HTTPS`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    /// The protocol version. Valid Values are `HTTP1` | `HTTP2` | `GRPC`. Default value is `HTTP1`.
    #[builder(into)]
    #[serde(rename = "protocolVersion")]
    pub r#protocol_version: Option<String>,
    /// The ID of the VPC.
    #[builder(into)]
    #[serde(rename = "vpcIdentifier")]
    pub r#vpc_identifier: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TargetGroupConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("health_check".to_string(), self.r#health_check.to_pulumi_value().await);
            map.insert("ip_address_type".to_string(), self.r#ip_address_type.to_pulumi_value().await);
            map.insert("lambda_event_structure_version".to_string(), self.r#lambda_event_structure_version.to_pulumi_value().await);
            map.insert("port".to_string(), self.r#port.to_pulumi_value().await);
            map.insert("protocol".to_string(), self.r#protocol.to_pulumi_value().await);
            map.insert("protocol_version".to_string(), self.r#protocol_version.to_pulumi_value().await);
            map.insert("vpc_identifier".to_string(), self.r#vpc_identifier.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TargetGroupConfig {
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
                    r#health_check: {
                        let field_value = match fields_map.get("health_check") {
                            Some(value) => value,
                            None => bail!("Missing field 'health_check' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::vpclattice::TargetGroupConfigHealthCheck>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ip_address_type: {
                        let field_value = match fields_map.get("ip_address_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_address_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#lambda_event_structure_version: {
                        let field_value = match fields_map.get("lambda_event_structure_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda_event_structure_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#protocol_version: {
                        let field_value = match fields_map.get("protocol_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vpc_identifier: {
                        let field_value = match fields_map.get("vpc_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
