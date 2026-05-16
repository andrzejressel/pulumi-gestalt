#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceNetworkConfiguration {
    /// Network configuration settings for outbound message traffic. See Egress Configuration below for more details.
    #[builder(into)]
    #[serde(rename = "egressConfiguration")]
    pub r#egress_configuration: Option<Box<super::super::types::apprunner::ServiceNetworkConfigurationEgressConfiguration>>,
    /// Network configuration settings for inbound network traffic. See Ingress Configuration below for more details.
    #[builder(into)]
    #[serde(rename = "ingressConfiguration")]
    pub r#ingress_configuration: Option<Box<super::super::types::apprunner::ServiceNetworkConfigurationIngressConfiguration>>,
    /// App Runner provides you with the option to choose between Internet Protocol version 4 (IPv4) and dual stack (IPv4 and IPv6) for your incoming public network configuration. Valid values: `IPV4`, `DUAL_STACK`. Default: `IPV4`.
    #[builder(into)]
    #[serde(rename = "ipAddressType")]
    pub r#ip_address_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceNetworkConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("egress_configuration".to_string(), self.r#egress_configuration.to_pulumi_value().await);
            map.insert("ingress_configuration".to_string(), self.r#ingress_configuration.to_pulumi_value().await);
            map.insert("ip_address_type".to_string(), self.r#ip_address_type.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceNetworkConfiguration {
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
                    r#egress_configuration: {
                        let field_value = match fields_map.get("egress_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'egress_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::apprunner::ServiceNetworkConfigurationEgressConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ingress_configuration: {
                        let field_value = match fields_map.get("ingress_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::apprunner::ServiceNetworkConfigurationIngressConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ip_address_type: {
                        let field_value = match fields_map.get("ip_address_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_address_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
