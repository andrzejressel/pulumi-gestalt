#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SingleNodeVirtualInstanceSingleServerConfiguration {
    #[builder(into)]
    #[serde(rename = "appResourceGroupName")]
    pub r#app_resource_group_name: String,
    /// The supported SAP database type. Possible values are `DB2` and `HANA`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "databaseType")]
    pub r#database_type: Option<String>,
    /// One or more `disk_volume_configuration` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diskVolumeConfigurations")]
    pub r#disk_volume_configurations: Option<Vec<super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfigurationDiskVolumeConfiguration>>,
    /// Specifies whether a secondary IP address should be added to the network interface on all VMs of the SAP system being deployed. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "secondaryIpEnabled")]
    pub r#secondary_ip_enabled: Option<bool>,
    /// The resource ID of the Subnet for the SAP Single Node Virtual Instance. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
    /// A `virtual_machine_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachineConfiguration")]
    pub r#virtual_machine_configuration: Box<super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineConfiguration>,
    /// A `virtual_machine_resource_names` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachineResourceNames")]
    pub r#virtual_machine_resource_names: Option<Box<super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineResourceNames>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SingleNodeVirtualInstanceSingleServerConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "app_resource_group_name",
                    &self.r#app_resource_group_name,
                ),
                to_pulumi_object_field(
                    "database_type",
                    &self.r#database_type,
                ),
                to_pulumi_object_field(
                    "disk_volume_configurations",
                    &self.r#disk_volume_configurations,
                ),
                to_pulumi_object_field(
                    "secondary_ip_enabled",
                    &self.r#secondary_ip_enabled,
                ),
                to_pulumi_object_field(
                    "subnet_id",
                    &self.r#subnet_id,
                ),
                to_pulumi_object_field(
                    "virtual_machine_configuration",
                    &self.r#virtual_machine_configuration,
                ),
                to_pulumi_object_field(
                    "virtual_machine_resource_names",
                    &self.r#virtual_machine_resource_names,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SingleNodeVirtualInstanceSingleServerConfiguration {
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
                    r#app_resource_group_name: {
                        let field_value = match fields_map.get("app_resource_group_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_resource_group_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_type: {
                        let field_value = match fields_map.get("database_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_volume_configurations: {
                        let field_value = match fields_map.get("disk_volume_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_volume_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_ip_enabled: {
                        let field_value = match fields_map.get("secondary_ip_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_ip_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#virtual_machine_configuration: {
                        let field_value = match fields_map.get("virtual_machine_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_machine_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_machine_resource_names: {
                        let field_value = match fields_map.get("virtual_machine_resource_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_machine_resource_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
