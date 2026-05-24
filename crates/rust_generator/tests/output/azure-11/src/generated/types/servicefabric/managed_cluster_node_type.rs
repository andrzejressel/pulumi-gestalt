#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedClusterNodeType {
    /// Sets the port range available for applications. Format is `<from_port>-<to_port>`, for example `10000-20000`.
    #[builder(into)]
    pub r#application_port_range: String,
    /// Specifies a list of key/value pairs used to set capacity tags for this node type.
    #[builder(into)]
    pub r#capacities: Option<std::collections::BTreeMap<String, String>>,
    /// The size of the data disk in gigabytes..
    #[builder(into)]
    pub r#data_disk_size_gb: i32,
    /// The type of the disk to use for storing data. It can be one of `Premium_LRS`, `Standard_LRS`, or `StandardSSD_LRS`. Defaults to `Standard_LRS`.
    #[builder(into)]
    pub r#data_disk_type: Option<String>,
    /// Sets the port range available for the OS. Format is `<from_port>-<to_port>`, for example `10000-20000`. There has to be at least 255 ports available and cannot overlap with `application_port_range`..
    #[builder(into)]
    pub r#ephemeral_port_range: String,
    /// The ID of the Resource Group.
    #[builder(into)]
    pub r#id: Option<String>,
    /// If set the node type can be composed of multiple placement groups.
    #[builder(into)]
    pub r#multiple_placement_groups_enabled: Option<bool>,
    /// The name which should be used for this node type.
    #[builder(into)]
    pub r#name: String,
    /// Specifies a list of placement tags that can be used to indicate where services should run..
    #[builder(into)]
    pub r#placement_properties: Option<std::collections::BTreeMap<String, String>>,
    /// If set to true, system services will run on this node type. Only one node type should be marked as primary. Primary node type cannot be deleted or changed once they're created.
    #[builder(into)]
    pub r#primary: Option<bool>,
    /// If set to true, only stateless workloads can run on this node type.
    #[builder(into)]
    pub r#stateless: Option<bool>,
    /// The offer type of the marketplace image cluster VMs will use.
    #[builder(into)]
    pub r#vm_image_offer: String,
    /// The publisher of the marketplace image cluster VMs will use.
    #[builder(into)]
    pub r#vm_image_publisher: String,
    /// The SKU of the marketplace image cluster VMs will use.
    #[builder(into)]
    pub r#vm_image_sku: String,
    /// The version of the marketplace image cluster VMs will use.
    #[builder(into)]
    pub r#vm_image_version: String,
    /// The number of instances this node type will launch.
    #[builder(into)]
    pub r#vm_instance_count: i32,
    /// One or more `vm_secrets` blocks as defined below.
    #[builder(into)]
    pub r#vm_secrets: Option<Vec<super::super::types::servicefabric::ManagedClusterNodeTypeVmSecret>>,
    /// The size of the instances in this node type.
    #[builder(into)]
    pub r#vm_size: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagedClusterNodeType {
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
                    "applicationPortRange",
                    &self.r#application_port_range,
                ),
                to_pulumi_object_field(
                    "capacities",
                    &self.r#capacities,
                ),
                to_pulumi_object_field(
                    "dataDiskSizeGb",
                    &self.r#data_disk_size_gb,
                ),
                to_pulumi_object_field(
                    "dataDiskType",
                    &self.r#data_disk_type,
                ),
                to_pulumi_object_field(
                    "ephemeralPortRange",
                    &self.r#ephemeral_port_range,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "multiplePlacementGroupsEnabled",
                    &self.r#multiple_placement_groups_enabled,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "placementProperties",
                    &self.r#placement_properties,
                ),
                to_pulumi_object_field(
                    "primary",
                    &self.r#primary,
                ),
                to_pulumi_object_field(
                    "stateless",
                    &self.r#stateless,
                ),
                to_pulumi_object_field(
                    "vmImageOffer",
                    &self.r#vm_image_offer,
                ),
                to_pulumi_object_field(
                    "vmImagePublisher",
                    &self.r#vm_image_publisher,
                ),
                to_pulumi_object_field(
                    "vmImageSku",
                    &self.r#vm_image_sku,
                ),
                to_pulumi_object_field(
                    "vmImageVersion",
                    &self.r#vm_image_version,
                ),
                to_pulumi_object_field(
                    "vmInstanceCount",
                    &self.r#vm_instance_count,
                ),
                to_pulumi_object_field(
                    "vmSecrets",
                    &self.r#vm_secrets,
                ),
                to_pulumi_object_field(
                    "vmSize",
                    &self.r#vm_size,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ManagedClusterNodeType {
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
                    r#application_port_range: {
                        let field_value = match fields_map.get("applicationPortRange") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationPortRange' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#capacities: {
                        let field_value = match fields_map.get("capacities") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_disk_size_gb: {
                        let field_value = match fields_map.get("dataDiskSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataDiskSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_disk_type: {
                        let field_value = match fields_map.get("dataDiskType") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataDiskType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ephemeral_port_range: {
                        let field_value = match fields_map.get("ephemeralPortRange") {
                            Some(value) => value,
                            None => bail!("Missing field 'ephemeralPortRange' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multiple_placement_groups_enabled: {
                        let field_value = match fields_map.get("multiplePlacementGroupsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiplePlacementGroupsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#placement_properties: {
                        let field_value = match fields_map.get("placementProperties") {
                            Some(value) => value,
                            None => bail!("Missing field 'placementProperties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#stateless: {
                        let field_value = match fields_map.get("stateless") {
                            Some(value) => value,
                            None => bail!("Missing field 'stateless' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_image_offer: {
                        let field_value = match fields_map.get("vmImageOffer") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmImageOffer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_image_publisher: {
                        let field_value = match fields_map.get("vmImagePublisher") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmImagePublisher' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_image_sku: {
                        let field_value = match fields_map.get("vmImageSku") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmImageSku' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_image_version: {
                        let field_value = match fields_map.get("vmImageVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmImageVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_instance_count: {
                        let field_value = match fields_map.get("vmInstanceCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmInstanceCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_secrets: {
                        let field_value = match fields_map.get("vmSecrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmSecrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_size: {
                        let field_value = match fields_map.get("vmSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
