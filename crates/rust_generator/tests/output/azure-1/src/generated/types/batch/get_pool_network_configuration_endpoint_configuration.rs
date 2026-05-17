#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPoolNetworkConfigurationEndpointConfiguration {
    /// The port number on the compute node.
    #[builder(into)]
    #[serde(rename = "backendPort")]
    pub r#backend_port: i32,
    /// The range of external ports that are used to provide inbound access to the backendPort on the individual compute nodes in the format of `1000-1100`.
    #[builder(into)]
    #[serde(rename = "frontendPortRange")]
    pub r#frontend_port_range: String,
    /// The name of the user account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The list of network security group rules that are applied to the endpoint.
    #[builder(into)]
    #[serde(rename = "networkSecurityGroupRules")]
    pub r#network_security_group_rules: Vec<super::super::types::batch::GetPoolNetworkConfigurationEndpointConfigurationNetworkSecurityGroupRule>,
    /// The protocol of the endpoint.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPoolNetworkConfigurationEndpointConfiguration {
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
                "backend_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backend_port,
                )
                .await,
            );
            map.insert(
                "frontend_port_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#frontend_port_range,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "network_security_group_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_security_group_rules,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPoolNetworkConfigurationEndpointConfiguration {
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
                    r#backend_port: {
                        let field_value = match fields_map.get("backend_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frontend_port_range: {
                        let field_value = match fields_map.get("frontend_port_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'frontend_port_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_security_group_rules: {
                        let field_value = match fields_map.get("network_security_group_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_security_group_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
