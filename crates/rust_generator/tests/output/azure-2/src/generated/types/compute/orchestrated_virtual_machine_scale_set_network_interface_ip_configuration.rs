#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfiguration {
    /// A list of Backend Address Pools IDs from a Application Gateway which this Virtual Machine Scale Set should be connected to.
    #[builder(into)]
    pub r#application_gateway_backend_address_pool_ids: Option<Vec<String>>,
    /// A list of Application Security Group IDs which this Virtual Machine Scale Set should be connected to.
    #[builder(into)]
    pub r#application_security_group_ids: Option<Vec<String>>,
    /// A list of Backend Address Pools IDs from a Load Balancer which this Virtual Machine Scale Set should be connected to.
    /// 
    /// > **Note:** When using this field you'll also need to configure a Rule for the Load Balancer, and use a depends_on between this resource and the Load Balancer Rule.
    #[builder(into)]
    pub r#load_balancer_backend_address_pool_ids: Option<Vec<String>>,
    /// The Name which should be used for this IP Configuration.
    #[builder(into)]
    pub r#name: String,
    /// Is this the Primary IP Configuration for this Network Interface? Possible values are `true` and `false`. Defaults to `false`.
    /// 
    /// > **Note:** One `ip_configuration` block must be marked as Primary for each Network Interface.
    #[builder(into)]
    pub r#primary: Option<bool>,
    /// A `public_ip_address` block as defined below.
    #[builder(into)]
    pub r#public_ip_addresses: Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddress>>,
    /// The ID of the Subnet which this IP Configuration should be connected to.
    /// 
    /// > **Note:** `subnet_id` is required if version is set to `IPv4`.
    #[builder(into)]
    pub r#subnet_id: Option<String>,
    /// The Internet Protocol Version which should be used for this IP Configuration. Possible values are `IPv4` and `IPv6`. Defaults to `IPv4`.
    #[builder(into)]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfiguration {
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
                    "applicationGatewayBackendAddressPoolIds",
                    &self.r#application_gateway_backend_address_pool_ids,
                ),
                to_pulumi_object_field(
                    "applicationSecurityGroupIds",
                    &self.r#application_security_group_ids,
                ),
                to_pulumi_object_field(
                    "loadBalancerBackendAddressPoolIds",
                    &self.r#load_balancer_backend_address_pool_ids,
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
                    "publicIpAddresses",
                    &self.r#public_ip_addresses,
                ),
                to_pulumi_object_field(
                    "subnetId",
                    &self.r#subnet_id,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfiguration {
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
                        let field_value = match fields_map.get("applicationGatewayBackendAddressPoolIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationGatewayBackendAddressPoolIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_security_group_ids: {
                        let field_value = match fields_map.get("applicationSecurityGroupIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationSecurityGroupIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_backend_address_pool_ids: {
                        let field_value = match fields_map.get("loadBalancerBackendAddressPoolIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancerBackendAddressPoolIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("publicIpAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
