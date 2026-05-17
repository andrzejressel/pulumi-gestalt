#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedClusterNodeType {
    /// Sets the port range available for applications. Format is `<from_port>-<to_port>`, for example `10000-20000`.
    #[builder(into)]
    #[serde(rename = "applicationPortRange")]
    pub r#application_port_range: String,
    /// Specifies a list of key/value pairs used to set capacity tags for this node type.
    #[builder(into)]
    #[serde(rename = "capacities")]
    pub r#capacities: Option<std::collections::HashMap<String, String>>,
    /// The size of the data disk in gigabytes..
    #[builder(into)]
    #[serde(rename = "dataDiskSizeGb")]
    pub r#data_disk_size_gb: i32,
    /// The type of the disk to use for storing data. It can be one of `Premium_LRS`, `Standard_LRS`, or `StandardSSD_LRS`. Defaults to `Standard_LRS`.
    #[builder(into)]
    #[serde(rename = "dataDiskType")]
    pub r#data_disk_type: Option<String>,
    /// Sets the port range available for the OS. Format is `<from_port>-<to_port>`, for example `10000-20000`. There has to be at least 255 ports available and cannot overlap with `application_port_range`..
    #[builder(into)]
    #[serde(rename = "ephemeralPortRange")]
    pub r#ephemeral_port_range: String,
    /// The ID of the Resource Group.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// If set the node type can be composed of multiple placement groups.
    #[builder(into)]
    #[serde(rename = "multiplePlacementGroupsEnabled")]
    pub r#multiple_placement_groups_enabled: Option<bool>,
    /// The name which should be used for this node type.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies a list of placement tags that can be used to indicate where services should run..
    #[builder(into)]
    #[serde(rename = "placementProperties")]
    pub r#placement_properties: Option<std::collections::HashMap<String, String>>,
    /// If set to true, system services will run on this node type. Only one node type should be marked as primary. Primary node type cannot be deleted or changed once they're created.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Option<bool>,
    /// If set to true, only stateless workloads can run on this node type.
    #[builder(into)]
    #[serde(rename = "stateless")]
    pub r#stateless: Option<bool>,
    /// The offer type of the marketplace image cluster VMs will use.
    #[builder(into)]
    #[serde(rename = "vmImageOffer")]
    pub r#vm_image_offer: String,
    /// The publisher of the marketplace image cluster VMs will use.
    #[builder(into)]
    #[serde(rename = "vmImagePublisher")]
    pub r#vm_image_publisher: String,
    /// The SKU of the marketplace image cluster VMs will use.
    #[builder(into)]
    #[serde(rename = "vmImageSku")]
    pub r#vm_image_sku: String,
    /// The version of the marketplace image cluster VMs will use.
    #[builder(into)]
    #[serde(rename = "vmImageVersion")]
    pub r#vm_image_version: String,
    /// The number of instances this node type will launch.
    #[builder(into)]
    #[serde(rename = "vmInstanceCount")]
    pub r#vm_instance_count: i32,
    /// One or more `vm_secrets` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "vmSecrets")]
    pub r#vm_secrets: Option<Vec<super::super::types::servicefabric::ManagedClusterNodeTypeVmSecret>>,
    /// The size of the instances in this node type.
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagedClusterNodeType {
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
                    "application_port_range",
                    &self.r#application_port_range,
                ),
                to_pulumi_object_field(
                    "capacities",
                    &self.r#capacities,
                ),
                to_pulumi_object_field(
                    "data_disk_size_gb",
                    &self.r#data_disk_size_gb,
                ),
                to_pulumi_object_field(
                    "data_disk_type",
                    &self.r#data_disk_type,
                ),
                to_pulumi_object_field(
                    "ephemeral_port_range",
                    &self.r#ephemeral_port_range,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "multiple_placement_groups_enabled",
                    &self.r#multiple_placement_groups_enabled,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "placement_properties",
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
                    "vm_image_offer",
                    &self.r#vm_image_offer,
                ),
                to_pulumi_object_field(
                    "vm_image_publisher",
                    &self.r#vm_image_publisher,
                ),
                to_pulumi_object_field(
                    "vm_image_sku",
                    &self.r#vm_image_sku,
                ),
                to_pulumi_object_field(
                    "vm_image_version",
                    &self.r#vm_image_version,
                ),
                to_pulumi_object_field(
                    "vm_instance_count",
                    &self.r#vm_instance_count,
                ),
                to_pulumi_object_field(
                    "vm_secrets",
                    &self.r#vm_secrets,
                ),
                to_pulumi_object_field(
                    "vm_size",
                    &self.r#vm_size,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("application_port_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_port_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("data_disk_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_disk_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_disk_type: {
                        let field_value = match fields_map.get("data_disk_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_disk_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ephemeral_port_range: {
                        let field_value = match fields_map.get("ephemeral_port_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'ephemeral_port_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("multiple_placement_groups_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiple_placement_groups_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("placement_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'placement_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("vm_image_offer") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_image_offer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_image_publisher: {
                        let field_value = match fields_map.get("vm_image_publisher") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_image_publisher' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_image_sku: {
                        let field_value = match fields_map.get("vm_image_sku") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_image_sku' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_image_version: {
                        let field_value = match fields_map.get("vm_image_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_image_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_instance_count: {
                        let field_value = match fields_map.get("vm_instance_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_instance_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_secrets: {
                        let field_value = match fields_map.get("vm_secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_size: {
                        let field_value = match fields_map.get("vm_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
