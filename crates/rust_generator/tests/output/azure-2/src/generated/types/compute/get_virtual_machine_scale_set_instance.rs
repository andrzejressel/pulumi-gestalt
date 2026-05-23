#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualMachineScaleSetInstance {
    /// The Hostname of this Virtual Machine.
    #[builder(into)]
    pub r#computer_name: String,
    /// The Instance ID of this Virtual Machine.
    #[builder(into)]
    pub r#instance_id: String,
    /// Whether the latest model has been applied to this Virtual Machine.
    #[builder(into)]
    pub r#latest_model_applied: bool,
    /// The name of this Virtual Machine Scale Set.
    #[builder(into)]
    pub r#name: String,
    /// The power state of the virtual machine.
    #[builder(into)]
    pub r#power_state: String,
    /// The Primary Private IP Address assigned to this Virtual Machine.
    #[builder(into)]
    pub r#private_ip_address: String,
    /// A list of Private IP Addresses assigned to this Virtual Machine.
    #[builder(into)]
    pub r#private_ip_addresses: Vec<String>,
    /// The virtual machines scale set IP Configuration's PublicIPAddress configuration. The `public_ip_address` is documented below.
    #[builder(into)]
    pub r#public_ip_address: String,
    /// A list of the Public IP Addresses assigned to this Virtual Machine.
    #[builder(into)]
    pub r#public_ip_addresses: Vec<String>,
    /// The unique ID of the virtual machine.
    #[builder(into)]
    pub r#virtual_machine_id: String,
    /// The zones of the virtual machine.
    #[builder(into)]
    pub r#zone: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualMachineScaleSetInstance {
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
                    "computerName",
                    &self.r#computer_name,
                ),
                to_pulumi_object_field(
                    "instanceId",
                    &self.r#instance_id,
                ),
                to_pulumi_object_field(
                    "latestModelApplied",
                    &self.r#latest_model_applied,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "powerState",
                    &self.r#power_state,
                ),
                to_pulumi_object_field(
                    "privateIpAddress",
                    &self.r#private_ip_address,
                ),
                to_pulumi_object_field(
                    "privateIpAddresses",
                    &self.r#private_ip_addresses,
                ),
                to_pulumi_object_field(
                    "publicIpAddress",
                    &self.r#public_ip_address,
                ),
                to_pulumi_object_field(
                    "publicIpAddresses",
                    &self.r#public_ip_addresses,
                ),
                to_pulumi_object_field(
                    "virtualMachineId",
                    &self.r#virtual_machine_id,
                ),
                to_pulumi_object_field(
                    "zone",
                    &self.r#zone,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualMachineScaleSetInstance {
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
                    r#computer_name: {
                        let field_value = match fields_map.get("computerName") {
                            Some(value) => value,
                            None => bail!("Missing field 'computerName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_id: {
                        let field_value = match fields_map.get("instanceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#latest_model_applied: {
                        let field_value = match fields_map.get("latestModelApplied") {
                            Some(value) => value,
                            None => bail!("Missing field 'latestModelApplied' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#power_state: {
                        let field_value = match fields_map.get("powerState") {
                            Some(value) => value,
                            None => bail!("Missing field 'powerState' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_address: {
                        let field_value = match fields_map.get("privateIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_addresses: {
                        let field_value = match fields_map.get("privateIpAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateIpAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_address: {
                        let field_value = match fields_map.get("publicIpAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#virtual_machine_id: {
                        let field_value = match fields_map.get("virtualMachineId") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtualMachineId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zone: {
                        let field_value = match fields_map.get("zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
