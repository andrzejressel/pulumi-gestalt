#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkflowTemplatePlacementManagedClusterConfigMasterConfig {
    /// The Compute Engine accelerator configuration for these instances.
    #[builder(into)]
    pub r#accelerators: Option<Vec<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigMasterConfigAccelerator>>,
    /// Disk option config settings.
    #[builder(into)]
    pub r#disk_config: Option<Box<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigMasterConfigDiskConfig>>,
    /// The Compute Engine image resource used for cluster instances. The URI can represent an image or image family. Image examples: * `https://www.googleapis.com/compute/beta/projects/` If the URI is unspecified, it will be inferred from `SoftwareConfig.image_version` or the system default.
    #[builder(into)]
    pub r#image: Option<String>,
    /// Output only. The list of instance names. Dataproc derives the names from `cluster_name`, `num_instances`, and the instance group.
    #[builder(into)]
    pub r#instance_names: Option<Vec<String>>,
    /// Output only. Specifies that this instance group contains preemptible instances.
    #[builder(into)]
    pub r#is_preemptible: Option<bool>,
    /// The Compute Engine machine type used for cluster instances. A full URL, partial URI, or short name are valid. Examples: * `https://www.googleapis.com/compute/v1/projects/(https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the machine type resource, for example, `n1-standard-2`.
    #[builder(into)]
    pub r#machine_type: Option<String>,
    /// Output only. The config for Compute Engine Instance Group Manager that manages this group. This is only used for preemptible instance groups.
    #[builder(into)]
    pub r#managed_group_configs: Option<Vec<super::super::types::dataproc::WorkflowTemplatePlacementManagedClusterConfigMasterConfigManagedGroupConfig>>,
    /// Specifies the minimum cpu platform for the Instance Group. See [Minimum CPU platform](https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu).
    #[builder(into)]
    pub r#min_cpu_platform: Option<String>,
    /// The number of VM instances in the instance group. For master instance groups, must be set to 1.
    #[builder(into)]
    pub r#num_instances: Option<i32>,
    /// Specifies the preemptibility of the instance group. The default value for master and worker groups is `NON_PREEMPTIBLE`. This default cannot be changed. The default value for secondary instances is `PREEMPTIBLE`. Possible values: PREEMPTIBILITY_UNSPECIFIED, NON_PREEMPTIBLE, PREEMPTIBLE
    #[builder(into)]
    pub r#preemptibility: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkflowTemplatePlacementManagedClusterConfigMasterConfig {
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
                    "accelerators",
                    &self.r#accelerators,
                ),
                to_pulumi_object_field(
                    "diskConfig",
                    &self.r#disk_config,
                ),
                to_pulumi_object_field(
                    "image",
                    &self.r#image,
                ),
                to_pulumi_object_field(
                    "instanceNames",
                    &self.r#instance_names,
                ),
                to_pulumi_object_field(
                    "isPreemptible",
                    &self.r#is_preemptible,
                ),
                to_pulumi_object_field(
                    "machineType",
                    &self.r#machine_type,
                ),
                to_pulumi_object_field(
                    "managedGroupConfigs",
                    &self.r#managed_group_configs,
                ),
                to_pulumi_object_field(
                    "minCpuPlatform",
                    &self.r#min_cpu_platform,
                ),
                to_pulumi_object_field(
                    "numInstances",
                    &self.r#num_instances,
                ),
                to_pulumi_object_field(
                    "preemptibility",
                    &self.r#preemptibility,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkflowTemplatePlacementManagedClusterConfigMasterConfig {
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
                    r#accelerators: {
                        let field_value = match fields_map.get("accelerators") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerators' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_config: {
                        let field_value = match fields_map.get("diskConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#instance_names: {
                        let field_value = match fields_map.get("instanceNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_preemptible: {
                        let field_value = match fields_map.get("isPreemptible") {
                            Some(value) => value,
                            None => bail!("Missing field 'isPreemptible' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_type: {
                        let field_value = match fields_map.get("machineType") {
                            Some(value) => value,
                            None => bail!("Missing field 'machineType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_group_configs: {
                        let field_value = match fields_map.get("managedGroupConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedGroupConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_cpu_platform: {
                        let field_value = match fields_map.get("minCpuPlatform") {
                            Some(value) => value,
                            None => bail!("Missing field 'minCpuPlatform' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#num_instances: {
                        let field_value = match fields_map.get("numInstances") {
                            Some(value) => value,
                            None => bail!("Missing field 'numInstances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preemptibility: {
                        let field_value = match fields_map.get("preemptibility") {
                            Some(value) => value,
                            None => bail!("Missing field 'preemptibility' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
