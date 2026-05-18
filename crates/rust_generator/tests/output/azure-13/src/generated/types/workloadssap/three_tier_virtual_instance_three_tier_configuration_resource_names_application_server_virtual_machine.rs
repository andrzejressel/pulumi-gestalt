#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServerVirtualMachine {
    /// One or more `data_disk` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "dataDisks")]
    pub r#data_disks: Option<Vec<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServerVirtualMachineDataDisk>>,
    /// The full name of the host of the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Option<String>,
    /// A list of full names for the Network Interface of the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "networkInterfaceNames")]
    pub r#network_interface_names: Option<Vec<String>>,
    /// The full name of the OS Disk attached to the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "osDiskName")]
    pub r#os_disk_name: Option<String>,
    /// The full name of the Virtual Machine in a single server SAP system. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachineName")]
    pub r#virtual_machine_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServerVirtualMachine {
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
                    "data_disks",
                    &self.r#data_disks,
                ),
                to_pulumi_object_field(
                    "host_name",
                    &self.r#host_name,
                ),
                to_pulumi_object_field(
                    "network_interface_names",
                    &self.r#network_interface_names,
                ),
                to_pulumi_object_field(
                    "os_disk_name",
                    &self.r#os_disk_name,
                ),
                to_pulumi_object_field(
                    "virtual_machine_name",
                    &self.r#virtual_machine_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServerVirtualMachine {
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
                    r#data_disks: {
                        let field_value = match fields_map.get("data_disks") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_disks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_name: {
                        let field_value = match fields_map.get("host_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interface_names: {
                        let field_value = match fields_map.get("network_interface_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_interface_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_disk_name: {
                        let field_value = match fields_map.get("os_disk_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_disk_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_machine_name: {
                        let field_value = match fields_map.get("virtual_machine_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_machine_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
