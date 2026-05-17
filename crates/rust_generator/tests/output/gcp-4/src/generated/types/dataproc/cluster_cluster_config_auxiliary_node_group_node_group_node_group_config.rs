#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterConfigAuxiliaryNodeGroupNodeGroupNodeGroupConfig {
    /// The Compute Engine accelerator (GPU) configuration for these instances. Can be specified 
    /// multiple times.
    #[builder(into)]
    #[serde(rename = "accelerators")]
    pub r#accelerators: Option<Vec<super::super::types::dataproc::ClusterClusterConfigAuxiliaryNodeGroupNodeGroupNodeGroupConfigAccelerator>>,
    /// Disk Config
    #[builder(into)]
    #[serde(rename = "diskConfig")]
    pub r#disk_config: Option<Box<super::super::types::dataproc::ClusterClusterConfigAuxiliaryNodeGroupNodeGroupNodeGroupConfigDiskConfig>>,
    /// List of auxiliary node group instance names which have been assigned to the cluster.
    #[builder(into)]
    #[serde(rename = "instanceNames")]
    pub r#instance_names: Option<Vec<String>>,
    /// The name of a Google Compute Engine machine type
    /// to create for the node group. If not specified, GCP will default to a predetermined
    /// computed value (currently `n1-standard-4`).
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Option<String>,
    /// The name of a minimum generation of CPU family
    /// for the node group. If not specified, GCP will default to a predetermined computed value
    /// for each zone. See [the guide](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform)
    /// for details about which CPU families are available (and defaulted) for each zone.
    #[builder(into)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Option<String>,
    /// Specifies the number of master nodes to create.
    /// Please set a number greater than 0. Node Group must have at least 1 instance.
    #[builder(into)]
    #[serde(rename = "numInstances")]
    pub r#num_instances: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterConfigAuxiliaryNodeGroupNodeGroupNodeGroupConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "accelerators".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accelerators,
                )
                .await,
            );
            map.insert(
                "disk_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disk_config,
                )
                .await,
            );
            map.insert(
                "instance_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_names,
                )
                .await,
            );
            map.insert(
                "machine_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#machine_type,
                )
                .await,
            );
            map.insert(
                "min_cpu_platform".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_cpu_platform,
                )
                .await,
            );
            map.insert(
                "num_instances".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#num_instances,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterConfigAuxiliaryNodeGroupNodeGroupNodeGroupConfig {
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
                        let field_value = match fields_map.get("disk_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_names: {
                        let field_value = match fields_map.get("instance_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_type: {
                        let field_value = match fields_map.get("machine_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_cpu_platform: {
                        let field_value = match fields_map.get("min_cpu_platform") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_cpu_platform' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#num_instances: {
                        let field_value = match fields_map.get("num_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
