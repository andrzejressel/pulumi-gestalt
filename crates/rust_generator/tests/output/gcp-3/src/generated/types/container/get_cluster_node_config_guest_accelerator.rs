#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodeConfigGuestAccelerator {
    /// The number of the accelerator cards exposed to an instance.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// Configuration for auto installation of GPU driver.
    #[builder(into)]
    #[serde(rename = "gpuDriverInstallationConfigs")]
    pub r#gpu_driver_installation_configs: Vec<super::super::types::container::GetClusterNodeConfigGuestAcceleratorGpuDriverInstallationConfig>,
    /// Size of partitions to create on the GPU. Valid values are described in the NVIDIA mig user guide (https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning)
    #[builder(into)]
    #[serde(rename = "gpuPartitionSize")]
    pub r#gpu_partition_size: String,
    /// Configuration for GPU sharing.
    #[builder(into)]
    #[serde(rename = "gpuSharingConfigs")]
    pub r#gpu_sharing_configs: Vec<super::super::types::container::GetClusterNodeConfigGuestAcceleratorGpuSharingConfig>,
    /// The accelerator type resource name.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterNodeConfigGuestAccelerator {
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
                "count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#count,
                )
                .await,
            );
            map.insert(
                "gpu_driver_installation_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gpu_driver_installation_configs,
                )
                .await,
            );
            map.insert(
                "gpu_partition_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gpu_partition_size,
                )
                .await,
            );
            map.insert(
                "gpu_sharing_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gpu_sharing_configs,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterNodeConfigGuestAccelerator {
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
                    r#count: {
                        let field_value = match fields_map.get("count") {
                            Some(value) => value,
                            None => bail!("Missing field 'count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gpu_driver_installation_configs: {
                        let field_value = match fields_map.get("gpu_driver_installation_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'gpu_driver_installation_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gpu_partition_size: {
                        let field_value = match fields_map.get("gpu_partition_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'gpu_partition_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gpu_sharing_configs: {
                        let field_value = match fields_map.get("gpu_sharing_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'gpu_sharing_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
