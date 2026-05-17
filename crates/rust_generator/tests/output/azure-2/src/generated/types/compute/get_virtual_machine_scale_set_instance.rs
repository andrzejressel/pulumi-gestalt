#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualMachineScaleSetInstance {
    /// The Hostname of this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "computerName")]
    pub r#computer_name: String,
    /// The Instance ID of this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: String,
    /// Whether the latest model has been applied to this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "latestModelApplied")]
    pub r#latest_model_applied: bool,
    /// The name of this Virtual Machine Scale Set.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The power state of the virtual machine.
    #[builder(into)]
    #[serde(rename = "powerState")]
    pub r#power_state: String,
    /// The Primary Private IP Address assigned to this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: String,
    /// A list of Private IP Addresses assigned to this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "privateIpAddresses")]
    pub r#private_ip_addresses: Vec<String>,
    /// The virtual machines scale set IP Configuration's PublicIPAddress configuration. The `public_ip_address` is documented below.
    #[builder(into)]
    #[serde(rename = "publicIpAddress")]
    pub r#public_ip_address: String,
    /// A list of the Public IP Addresses assigned to this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Vec<String>,
    /// The unique ID of the virtual machine.
    #[builder(into)]
    #[serde(rename = "virtualMachineId")]
    pub r#virtual_machine_id: String,
    /// The zones of the virtual machine.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualMachineScaleSetInstance {
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
                    "computer_name",
                    &self.r#computer_name,
                ),
                to_pulumi_object_field(
                    "instance_id",
                    &self.r#instance_id,
                ),
                to_pulumi_object_field(
                    "latest_model_applied",
                    &self.r#latest_model_applied,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "power_state",
                    &self.r#power_state,
                ),
                to_pulumi_object_field(
                    "private_ip_address",
                    &self.r#private_ip_address,
                ),
                to_pulumi_object_field(
                    "private_ip_addresses",
                    &self.r#private_ip_addresses,
                ),
                to_pulumi_object_field(
                    "public_ip_address",
                    &self.r#public_ip_address,
                ),
                to_pulumi_object_field(
                    "public_ip_addresses",
                    &self.r#public_ip_addresses,
                ),
                to_pulumi_object_field(
                    "virtual_machine_id",
                    &self.r#virtual_machine_id,
                ),
                to_pulumi_object_field(
                    "zone",
                    &self.r#zone,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("computer_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'computer_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_id: {
                        let field_value = match fields_map.get("instance_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#latest_model_applied: {
                        let field_value = match fields_map.get("latest_model_applied") {
                            Some(value) => value,
                            None => bail!("Missing field 'latest_model_applied' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("power_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'power_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_address: {
                        let field_value = match fields_map.get("private_ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_ip_addresses: {
                        let field_value = match fields_map.get("private_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_address: {
                        let field_value = match fields_map.get("public_ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#virtual_machine_id: {
                        let field_value = match fields_map.get("virtual_machine_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_machine_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
