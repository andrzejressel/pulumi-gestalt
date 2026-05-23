#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareNodePoolConfig {
    /// VMware disk size to be used during creation.
    #[builder(into)]
    pub r#boot_disk_size_gb: Option<i32>,
    /// The number of CPUs for each node in the node pool.
    #[builder(into)]
    pub r#cpus: Option<i32>,
    /// Allow node pool traffic to be load balanced. Only works for clusters with
    /// MetalLB load balancers.
    #[builder(into)]
    pub r#enable_load_balancer: Option<bool>,
    /// The OS image name in vCenter, only valid when using Windows.
    #[builder(into)]
    pub r#image: Option<String>,
    /// The OS image to be used for each node in a node pool.
    /// Currently `cos`, `cos_cgv2`, `ubuntu`, `ubuntu_cgv2`, `ubuntu_containerd` and `windows` are supported.
    #[builder(into)]
    pub r#image_type: String,
    /// The map of Kubernetes labels (key/value pairs) to be applied to each node.
    /// These will added in addition to any default label(s) that
    /// Kubernetes may apply to the node.
    /// In case of conflict in label keys, the applied set may differ depending on
    /// the Kubernetes version -- it's best to assume the behavior is undefined
    /// and conflicts should be avoided.
    #[builder(into)]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// The megabytes of memory for each node in the node pool.
    #[builder(into)]
    pub r#memory_mb: Option<i32>,
    /// The number of nodes in the node pool.
    #[builder(into)]
    pub r#replicas: Option<i32>,
    /// The initial taints assigned to nodes of this node pool.
    /// Structure is documented below.
    #[builder(into)]
    pub r#taints: Option<Vec<super::super::types::gkeonprem::VMwareNodePoolConfigTaint>>,
    /// Specifies the vSphere config for node pool.
    /// Structure is documented below.
    #[builder(into)]
    pub r#vsphere_config: Option<Box<super::super::types::gkeonprem::VMwareNodePoolConfigVsphereConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VMwareNodePoolConfig {
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
                    "bootDiskSizeGb",
                    &self.r#boot_disk_size_gb,
                ),
                to_pulumi_object_field(
                    "cpus",
                    &self.r#cpus,
                ),
                to_pulumi_object_field(
                    "enableLoadBalancer",
                    &self.r#enable_load_balancer,
                ),
                to_pulumi_object_field(
                    "image",
                    &self.r#image,
                ),
                to_pulumi_object_field(
                    "imageType",
                    &self.r#image_type,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "memoryMb",
                    &self.r#memory_mb,
                ),
                to_pulumi_object_field(
                    "replicas",
                    &self.r#replicas,
                ),
                to_pulumi_object_field(
                    "taints",
                    &self.r#taints,
                ),
                to_pulumi_object_field(
                    "vsphereConfig",
                    &self.r#vsphere_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VMwareNodePoolConfig {
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
                    r#boot_disk_size_gb: {
                        let field_value = match fields_map.get("bootDiskSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'bootDiskSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpus: {
                        let field_value = match fields_map.get("cpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_load_balancer: {
                        let field_value = match fields_map.get("enableLoadBalancer") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableLoadBalancer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image: {
                        let field_value = match fields_map.get("image") {
                            Some(value) => value,
                            None => bail!("Missing field 'image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_type: {
                        let field_value = match fields_map.get("imageType") {
                            Some(value) => value,
                            None => bail!("Missing field 'imageType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_mb: {
                        let field_value = match fields_map.get("memoryMb") {
                            Some(value) => value,
                            None => bail!("Missing field 'memoryMb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replicas: {
                        let field_value = match fields_map.get("replicas") {
                            Some(value) => value,
                            None => bail!("Missing field 'replicas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#taints: {
                        let field_value = match fields_map.get("taints") {
                            Some(value) => value,
                            None => bail!("Missing field 'taints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vsphere_config: {
                        let field_value = match fields_map.get("vsphereConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'vsphereConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
