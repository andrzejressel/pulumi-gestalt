#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetOrchestratedVirtualMachineScaleSetNetworkInterface {
    /// Is accelerated networking enabled?
    #[builder(into)]
    pub r#accelerated_networking_enabled: bool,
    /// An array of the DNS servers in use.
    #[builder(into)]
    pub r#dns_servers: Vec<String>,
    /// An `ip_configuration` block as documented below.
    #[builder(into)]
    pub r#ip_configurations: Vec<super::super::types::compute::GetOrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfiguration>,
    /// Is IP forwarding enabled?
    #[builder(into)]
    pub r#ip_forwarding_enabled: bool,
    /// The name of this Orchestrated Virtual Machine Scale Set.
    #[builder(into)]
    pub r#name: String,
    /// The identifier for the network security group.
    #[builder(into)]
    pub r#network_security_group_id: String,
    /// If this ip_configuration is the primary one.
    #[builder(into)]
    pub r#primary: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetOrchestratedVirtualMachineScaleSetNetworkInterface {
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
                    "acceleratedNetworkingEnabled",
                    &self.r#accelerated_networking_enabled,
                ),
                to_pulumi_object_field(
                    "dnsServers",
                    &self.r#dns_servers,
                ),
                to_pulumi_object_field(
                    "ipConfigurations",
                    &self.r#ip_configurations,
                ),
                to_pulumi_object_field(
                    "ipForwardingEnabled",
                    &self.r#ip_forwarding_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetOrchestratedVirtualMachineScaleSetNetworkInterface {
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
                    r#accelerated_networking_enabled: {
                        let field_value = match fields_map.get("acceleratedNetworkingEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'acceleratedNetworkingEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_servers: {
                        let field_value = match fields_map.get("dnsServers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dnsServers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ip_forwarding_enabled: {
                        let field_value = match fields_map.get("ipForwardingEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipForwardingEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
