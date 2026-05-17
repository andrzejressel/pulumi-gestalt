#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodePoolNodeConfigGuestAccelerator {
    /// The number of the guest accelerator cards exposed to this instance.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// Configuration for auto installation of GPU driver. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gpuDriverInstallationConfig")]
    pub r#gpu_driver_installation_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigGuestAcceleratorGpuDriverInstallationConfig>>,
    /// Size of partitions to create on the GPU. Valid values are described in the NVIDIA mig [user guide](https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning).
    #[builder(into)]
    #[serde(rename = "gpuPartitionSize")]
    pub r#gpu_partition_size: Option<String>,
    /// Configuration for GPU sharing. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "gpuSharingConfig")]
    pub r#gpu_sharing_config: Option<Box<super::super::types::container::ClusterNodePoolNodeConfigGuestAcceleratorGpuSharingConfig>>,
    /// The accelerator type resource to expose to this instance. E.g. `nvidia-tesla-k80`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNodePoolNodeConfigGuestAccelerator {
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
                    "count",
                    &self.r#count,
                ),
                to_pulumi_object_field(
                    "gpu_driver_installation_config",
                    &self.r#gpu_driver_installation_config,
                ),
                to_pulumi_object_field(
                    "gpu_partition_size",
                    &self.r#gpu_partition_size,
                ),
                to_pulumi_object_field(
                    "gpu_sharing_config",
                    &self.r#gpu_sharing_config,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNodePoolNodeConfigGuestAccelerator {
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
                    r#gpu_driver_installation_config: {
                        let field_value = match fields_map.get("gpu_driver_installation_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'gpu_driver_installation_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#gpu_sharing_config: {
                        let field_value = match fields_map.get("gpu_sharing_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'gpu_sharing_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
