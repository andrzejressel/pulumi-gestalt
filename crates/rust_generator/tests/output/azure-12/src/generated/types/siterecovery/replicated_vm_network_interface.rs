#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicatedVmNetworkInterface {
    /// Id of the public IP object to use when a test failover is done.
    #[builder(into)]
    #[serde(rename = "failoverTestPublicIpAddressId")]
    pub r#failover_test_public_ip_address_id: Option<String>,
    /// Static IP to assign when a test failover is done.
    #[builder(into)]
    #[serde(rename = "failoverTestStaticIp")]
    pub r#failover_test_static_ip: Option<String>,
    /// Name of the subnet to to use when a test failover is done.
    #[builder(into)]
    #[serde(rename = "failoverTestSubnetName")]
    pub r#failover_test_subnet_name: Option<String>,
    /// Id of the public IP object to use when a failover is done.
    #[builder(into)]
    #[serde(rename = "recoveryPublicIpAddressId")]
    pub r#recovery_public_ip_address_id: Option<String>,
    /// (Required if the network_interface block is specified) Id source network interface.
    #[builder(into)]
    #[serde(rename = "sourceNetworkInterfaceId")]
    pub r#source_network_interface_id: Option<String>,
    /// Static IP to assign when a failover is done.
    #[builder(into)]
    #[serde(rename = "targetStaticIp")]
    pub r#target_static_ip: Option<String>,
    /// Name of the subnet to to use when a failover is done.
    #[builder(into)]
    #[serde(rename = "targetSubnetName")]
    pub r#target_subnet_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicatedVmNetworkInterface {
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
                "failover_test_public_ip_address_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failover_test_public_ip_address_id,
                )
                .await,
            );
            map.insert(
                "failover_test_static_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failover_test_static_ip,
                )
                .await,
            );
            map.insert(
                "failover_test_subnet_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failover_test_subnet_name,
                )
                .await,
            );
            map.insert(
                "recovery_public_ip_address_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recovery_public_ip_address_id,
                )
                .await,
            );
            map.insert(
                "source_network_interface_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_network_interface_id,
                )
                .await,
            );
            map.insert(
                "target_static_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_static_ip,
                )
                .await,
            );
            map.insert(
                "target_subnet_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_subnet_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicatedVmNetworkInterface {
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
                    r#failover_test_public_ip_address_id: {
                        let field_value = match fields_map.get("failover_test_public_ip_address_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'failover_test_public_ip_address_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failover_test_static_ip: {
                        let field_value = match fields_map.get("failover_test_static_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'failover_test_static_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failover_test_subnet_name: {
                        let field_value = match fields_map.get("failover_test_subnet_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'failover_test_subnet_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recovery_public_ip_address_id: {
                        let field_value = match fields_map.get("recovery_public_ip_address_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'recovery_public_ip_address_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_network_interface_id: {
                        let field_value = match fields_map.get("source_network_interface_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_network_interface_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_static_ip: {
                        let field_value = match fields_map.get("target_static_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_static_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_subnet_name: {
                        let field_value = match fields_map.get("target_subnet_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_subnet_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
