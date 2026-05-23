#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxVirtualMachineScaleSetNetworkInterface {
    /// A list of IP Addresses of DNS Servers which should be assigned to the Network Interface.
    #[builder(into)]
    pub r#dns_servers: Option<Vec<String>>,
    /// Does this Network Interface support Accelerated Networking? Defaults to `false`.
    #[builder(into)]
    pub r#enable_accelerated_networking: Option<bool>,
    /// Does this Network Interface support IP Forwarding? Defaults to `false`.
    #[builder(into)]
    pub r#enable_ip_forwarding: Option<bool>,
    /// One or more `ip_configuration` blocks as defined above.
    #[builder(into)]
    pub r#ip_configurations: Vec<super::super::types::compute::LinuxVirtualMachineScaleSetNetworkInterfaceIpConfiguration>,
    /// The Name which should be used for this Network Interface. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#name: String,
    /// The ID of a Network Security Group which should be assigned to this Network Interface.
    #[builder(into)]
    pub r#network_security_group_id: Option<String>,
    /// Is this the Primary IP Configuration?
    /// 
    /// > **Note:** If multiple `network_interface` blocks are specified, one must be set to `primary`.
    #[builder(into)]
    pub r#primary: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxVirtualMachineScaleSetNetworkInterface {
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
                    "dnsServers",
                    &self.r#dns_servers,
                ),
                to_pulumi_object_field(
                    "enableAcceleratedNetworking",
                    &self.r#enable_accelerated_networking,
                ),
                to_pulumi_object_field(
                    "enableIpForwarding",
                    &self.r#enable_ip_forwarding,
                ),
                to_pulumi_object_field(
                    "ipConfigurations",
                    &self.r#ip_configurations,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "networkSecurityGroupId",
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxVirtualMachineScaleSetNetworkInterface {
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
                        let field_value = match fields_map.get("dnsServers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dnsServers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_accelerated_networking: {
                        let field_value = match fields_map.get("enableAcceleratedNetworking") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableAcceleratedNetworking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_ip_forwarding: {
                        let field_value = match fields_map.get("enableIpForwarding") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableIpForwarding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_configurations: {
                        let field_value = match fields_map.get("ipConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("networkSecurityGroupId") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkSecurityGroupId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
