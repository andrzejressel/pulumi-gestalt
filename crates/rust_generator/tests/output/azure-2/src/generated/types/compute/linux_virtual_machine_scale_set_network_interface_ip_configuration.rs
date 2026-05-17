#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxVirtualMachineScaleSetNetworkInterfaceIpConfiguration {
    /// A list of Backend Address Pools ID's from a Application Gateway which this Virtual Machine Scale Set should be connected to.
    #[builder(into)]
    #[serde(rename = "applicationGatewayBackendAddressPoolIds")]
    pub r#application_gateway_backend_address_pool_ids: Option<Vec<String>>,
    /// A list of Application Security Group ID's which this Virtual Machine Scale Set should be connected to.
    #[builder(into)]
    #[serde(rename = "applicationSecurityGroupIds")]
    pub r#application_security_group_ids: Option<Vec<String>>,
    /// A list of Backend Address Pools ID's from a Load Balancer which this Virtual Machine Scale Set should be connected to.
    /// 
    /// > **Note:**  When the Virtual Machine Scale Set is configured to have public IPs per instance are created with a load balancer, the SKU of the Virtual Machine instance IPs is determined by the SKU of the Virtual Machine Scale Sets Load Balancer (e.g. `Basic` or `Standard`). Alternatively, you may use the `public_ip_prefix_id` field to generate instance-level IPs in a virtual machine scale set as well. The zonal properties of the prefix will be passed to the Virtual Machine instance IPs, though they will not be shown in the output. To view the public IP addresses assigned to the Virtual Machine Scale Sets Virtual Machine instances use the **az vmss list-instance-public-ips --resource-group `ResourceGroupName` --name `VirtualMachineScaleSetName`** CLI command.
    /// 
    /// > **Note:** When using this field you'll also need to configure a Rule for the Load Balancer, and use a `depends_on` between this resource and the Load Balancer Rule.
    #[builder(into)]
    #[serde(rename = "loadBalancerBackendAddressPoolIds")]
    pub r#load_balancer_backend_address_pool_ids: Option<Vec<String>>,
    /// A list of NAT Rule ID's from a Load Balancer which this Virtual Machine Scale Set should be connected to.
    /// 
    /// > **Note:** When using this field you'll also need to configure a Rule for the Load Balancer, and use a `depends_on` between this resource and the Load Balancer Rule.
    #[builder(into)]
    #[serde(rename = "loadBalancerInboundNatRulesIds")]
    pub r#load_balancer_inbound_nat_rules_ids: Option<Vec<String>>,
    /// The Name which should be used for this IP Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Is this the Primary IP Configuration for this Network Interface? Defaults to `false`.
    /// 
    /// > **Note:** One `ip_configuration` block must be marked as Primary for each Network Interface.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Option<bool>,
    /// A `public_ip_address` block as defined below.
    #[builder(into)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Option<Vec<super::super::types::compute::LinuxVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddress>>,
    /// The ID of the Subnet which this IP Configuration should be connected to.
    /// 
    /// > `subnet_id` is required if `version` is set to `IPv4`.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
    /// The Internet Protocol Version which should be used for this IP Configuration. Possible values are `IPv4` and `IPv6`. Defaults to `IPv4`.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxVirtualMachineScaleSetNetworkInterfaceIpConfiguration {
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
                    "application_gateway_backend_address_pool_ids",
                    &self.r#application_gateway_backend_address_pool_ids,
                ),
                to_pulumi_object_field(
                    "application_security_group_ids",
                    &self.r#application_security_group_ids,
                ),
                to_pulumi_object_field(
                    "load_balancer_backend_address_pool_ids",
                    &self.r#load_balancer_backend_address_pool_ids,
                ),
                to_pulumi_object_field(
                    "load_balancer_inbound_nat_rules_ids",
                    &self.r#load_balancer_inbound_nat_rules_ids,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "primary",
                    &self.r#primary,
                ),
                to_pulumi_object_field(
                    "public_ip_addresses",
                    &self.r#public_ip_addresses,
                ),
                to_pulumi_object_field(
                    "subnet_id",
                    &self.r#subnet_id,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxVirtualMachineScaleSetNetworkInterfaceIpConfiguration {
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
                    r#application_gateway_backend_address_pool_ids: {
                        let field_value = match fields_map.get("application_gateway_backend_address_pool_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_gateway_backend_address_pool_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_security_group_ids: {
                        let field_value = match fields_map.get("application_security_group_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_security_group_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_backend_address_pool_ids: {
                        let field_value = match fields_map.get("load_balancer_backend_address_pool_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_backend_address_pool_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_inbound_nat_rules_ids: {
                        let field_value = match fields_map.get("load_balancer_inbound_nat_rules_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_inbound_nat_rules_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#primary: {
                        let field_value = match fields_map.get("primary") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_addresses: {
                        let field_value = match fields_map.get("public_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
