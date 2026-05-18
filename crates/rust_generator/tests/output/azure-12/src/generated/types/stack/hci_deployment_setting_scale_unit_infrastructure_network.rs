#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciDeploymentSettingScaleUnitInfrastructureNetwork {
    /// Whether DHCP is enabled for hosts and cluster IPs. Possible values are `true` and `false`. defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    /// 
    /// > **NOTE:** If `dhcp_enabled` is set to `false`, the deployment will use static IPs. If set to `true`, the gateway and DNS servers are not required.
    #[builder(into)]
    #[serde(rename = "dhcpEnabled")]
    pub r#dhcp_enabled: Option<bool>,
    /// Specifies a list of IPv4 addresses of the DNS servers in your environment. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Vec<String>,
    /// Specifies the default gateway that should be used for the provided IP address space. It should be in the format of an IPv4 IP address. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "gateway")]
    pub r#gateway: String,
    /// One or more `ip_pool` blocks as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "ipPools")]
    pub r#ip_pools: Vec<super::super::types::stack::HciDeploymentSettingScaleUnitInfrastructureNetworkIpPool>,
    /// Specifies the subnet mask that matches the provided IP address space. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "subnetMask")]
    pub r#subnet_mask: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HciDeploymentSettingScaleUnitInfrastructureNetwork {
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
                    "dhcp_enabled",
                    &self.r#dhcp_enabled,
                ),
                to_pulumi_object_field(
                    "dns_servers",
                    &self.r#dns_servers,
                ),
                to_pulumi_object_field(
                    "gateway",
                    &self.r#gateway,
                ),
                to_pulumi_object_field(
                    "ip_pools",
                    &self.r#ip_pools,
                ),
                to_pulumi_object_field(
                    "subnet_mask",
                    &self.r#subnet_mask,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HciDeploymentSettingScaleUnitInfrastructureNetwork {
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
                    r#dhcp_enabled: {
                        let field_value = match fields_map.get("dhcp_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'dhcp_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_servers: {
                        let field_value = match fields_map.get("dns_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gateway: {
                        let field_value = match fields_map.get("gateway") {
                            Some(value) => value,
                            None => bail!("Missing field 'gateway' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_pools: {
                        let field_value = match fields_map.get("ip_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_mask: {
                        let field_value = match fields_map.get("subnet_mask") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_mask' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
