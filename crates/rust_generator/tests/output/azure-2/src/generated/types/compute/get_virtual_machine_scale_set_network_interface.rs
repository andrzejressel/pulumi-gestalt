#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualMachineScaleSetNetworkInterface {
    /// An array of the DNS servers in use.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Vec<String>,
    /// Whether to enable accelerated networking or not.
    #[builder(into)]
    #[serde(rename = "enableAcceleratedNetworking")]
    pub r#enable_accelerated_networking: bool,
    /// Whether IP forwarding is enabled on this NIC.
    #[builder(into)]
    #[serde(rename = "enableIpForwarding")]
    pub r#enable_ip_forwarding: bool,
    /// An `ip_configuration` block as documented below.
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Vec<super::super::types::compute::GetVirtualMachineScaleSetNetworkInterfaceIpConfiguration>,
    /// The name of this Virtual Machine Scale Set.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The identifier for the network security group.
    #[builder(into)]
    #[serde(rename = "networkSecurityGroupId")]
    pub r#network_security_group_id: String,
    /// If this ip_configuration is the primary one.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualMachineScaleSetNetworkInterface {
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
                    "dns_servers",
                    &self.r#dns_servers,
                ),
                to_pulumi_object_field(
                    "enable_accelerated_networking",
                    &self.r#enable_accelerated_networking,
                ),
                to_pulumi_object_field(
                    "enable_ip_forwarding",
                    &self.r#enable_ip_forwarding,
                ),
                to_pulumi_object_field(
                    "ip_configurations",
                    &self.r#ip_configurations,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "network_security_group_id",
                    &self.r#network_security_group_id,
                ),
                to_pulumi_object_field(
                    "primary",
                    &self.r#primary,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualMachineScaleSetNetworkInterface {
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
                    r#dns_servers: {
                        let field_value = match fields_map.get("dns_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_accelerated_networking: {
                        let field_value = match fields_map.get("enable_accelerated_networking") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_accelerated_networking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_ip_forwarding: {
                        let field_value = match fields_map.get("enable_ip_forwarding") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_ip_forwarding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_configurations: {
                        let field_value = match fields_map.get("ip_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#network_security_group_id: {
                        let field_value = match fields_map.get("network_security_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_security_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary: {
                        let field_value = match fields_map.get("primary") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
