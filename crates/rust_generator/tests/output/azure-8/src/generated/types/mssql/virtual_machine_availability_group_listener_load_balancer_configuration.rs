#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineAvailabilityGroupListenerLoadBalancerConfiguration {
    /// The ID of the Load Balancer. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "loadBalancerId")]
    pub r#load_balancer_id: String,
    /// The private IP Address of the listener. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: String,
    /// The probe port of the listener. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "probePort")]
    pub r#probe_port: i32,
    /// Specifies a list of SQL Virtual Machine IDs. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sqlVirtualMachineIds")]
    pub r#sql_virtual_machine_ids: Vec<String>,
    /// The ID of the Subnet to create the listener. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** `sql_virtual_machine_ids` should match with the SQL Virtual Machines specified in `replica`.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineAvailabilityGroupListenerLoadBalancerConfiguration {
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
                    "load_balancer_id",
                    &self.r#load_balancer_id,
                ),
                to_pulumi_object_field(
                    "private_ip_address",
                    &self.r#private_ip_address,
                ),
                to_pulumi_object_field(
                    "probe_port",
                    &self.r#probe_port,
                ),
                to_pulumi_object_field(
                    "sql_virtual_machine_ids",
                    &self.r#sql_virtual_machine_ids,
                ),
                to_pulumi_object_field(
                    "subnet_id",
                    &self.r#subnet_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineAvailabilityGroupListenerLoadBalancerConfiguration {
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
                    r#load_balancer_id: {
                        let field_value = match fields_map.get("load_balancer_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#probe_port: {
                        let field_value = match fields_map.get("probe_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'probe_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_virtual_machine_ids: {
                        let field_value = match fields_map.get("sql_virtual_machine_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_virtual_machine_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
