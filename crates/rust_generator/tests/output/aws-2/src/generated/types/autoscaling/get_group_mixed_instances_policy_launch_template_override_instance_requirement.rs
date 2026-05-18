#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirement {
    #[builder(into)]
    #[serde(rename = "acceleratorCounts")]
    pub r#accelerator_counts: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementAcceleratorCount>,
    /// List of accelerator manufacturer names.
    #[builder(into)]
    #[serde(rename = "acceleratorManufacturers")]
    pub r#accelerator_manufacturers: Vec<String>,
    /// List of accelerator names.
    #[builder(into)]
    #[serde(rename = "acceleratorNames")]
    pub r#accelerator_names: Vec<String>,
    /// List of objects describing the minimum and maximum total memory of the accelerators.
    #[builder(into)]
    #[serde(rename = "acceleratorTotalMemoryMibs")]
    pub r#accelerator_total_memory_mibs: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementAcceleratorTotalMemoryMib>,
    /// List of accelerator types.
    #[builder(into)]
    #[serde(rename = "acceleratorTypes")]
    pub r#accelerator_types: Vec<String>,
    /// List of instance types to apply the specified attributes against.
    #[builder(into)]
    #[serde(rename = "allowedInstanceTypes")]
    pub r#allowed_instance_types: Vec<String>,
    /// Indicates whether bare metal instances are included, excluded, or required.
    #[builder(into)]
    #[serde(rename = "bareMetal")]
    pub r#bare_metal: String,
    /// List of objects describing the minimum and maximum baseline EBS bandwidth (Mbps).
    #[builder(into)]
    #[serde(rename = "baselineEbsBandwidthMbps")]
    pub r#baseline_ebs_bandwidth_mbps: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementBaselineEbsBandwidthMbp>,
    /// Indicates whether burstable performance instance types are included, excluded, or required.
    #[builder(into)]
    #[serde(rename = "burstablePerformance")]
    pub r#burstable_performance: String,
    /// List of CPU manufacturer names.
    #[builder(into)]
    #[serde(rename = "cpuManufacturers")]
    pub r#cpu_manufacturers: Vec<String>,
    /// List of excluded instance types.
    #[builder(into)]
    #[serde(rename = "excludedInstanceTypes")]
    pub r#excluded_instance_types: Vec<String>,
    /// List of instance generation names.
    #[builder(into)]
    #[serde(rename = "instanceGenerations")]
    pub r#instance_generations: Vec<String>,
    /// Indicates whether instance types with instance store volumes are included, excluded, or required.
    #[builder(into)]
    #[serde(rename = "localStorage")]
    pub r#local_storage: String,
    /// List of local storage type names.
    #[builder(into)]
    #[serde(rename = "localStorageTypes")]
    pub r#local_storage_types: Vec<String>,
    /// Price protection threshold for Spot Instances.
    #[builder(into)]
    #[serde(rename = "maxSpotPriceAsPercentageOfOptimalOnDemandPrice")]
    pub r#max_spot_price_as_percentage_of_optimal_on_demand_price: i32,
    /// List of objects describing the minimum and maximum amount of memory (GiB) per vCPU.
    #[builder(into)]
    #[serde(rename = "memoryGibPerVcpus")]
    pub r#memory_gib_per_vcpus: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementMemoryGibPerVcpus>,
    /// List of objects describing the minimum and maximum amount of memory (MiB).
    #[builder(into)]
    #[serde(rename = "memoryMibs")]
    pub r#memory_mibs: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementMemoryMib>,
    /// List of objects describing the minimum and maximum amount of network bandwidth (Gbps).
    #[builder(into)]
    #[serde(rename = "networkBandwidthGbps")]
    pub r#network_bandwidth_gbps: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementNetworkBandwidthGbp>,
    /// List of objects describing the minimum and maximum amount of network interfaces.
    #[builder(into)]
    #[serde(rename = "networkInterfaceCounts")]
    pub r#network_interface_counts: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementNetworkInterfaceCount>,
    /// Price protection threshold for On-Demand Instances.
    #[builder(into)]
    #[serde(rename = "onDemandMaxPricePercentageOverLowestPrice")]
    pub r#on_demand_max_price_percentage_over_lowest_price: i32,
    /// Indicates whether instance types must support On-Demand Instance Hibernation.
    #[builder(into)]
    #[serde(rename = "requireHibernateSupport")]
    pub r#require_hibernate_support: bool,
    /// Price protection threshold for Spot Instances.
    #[builder(into)]
    #[serde(rename = "spotMaxPricePercentageOverLowestPrice")]
    pub r#spot_max_price_percentage_over_lowest_price: i32,
    /// List of objects describing the minimum and maximum total storage (GB).
    #[builder(into)]
    #[serde(rename = "totalLocalStorageGbs")]
    pub r#total_local_storage_gbs: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementTotalLocalStorageGb>,
    /// List of objects describing the minimum and maximum number of vCPUs.
    #[builder(into)]
    #[serde(rename = "vcpuCounts")]
    pub r#vcpu_counts: Vec<super::super::types::autoscaling::GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementVcpuCount>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirement {
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
                    "accelerator_counts",
                    &self.r#accelerator_counts,
                ),
                to_pulumi_object_field(
                    "accelerator_manufacturers",
                    &self.r#accelerator_manufacturers,
                ),
                to_pulumi_object_field(
                    "accelerator_names",
                    &self.r#accelerator_names,
                ),
                to_pulumi_object_field(
                    "accelerator_total_memory_mibs",
                    &self.r#accelerator_total_memory_mibs,
                ),
                to_pulumi_object_field(
                    "accelerator_types",
                    &self.r#accelerator_types,
                ),
                to_pulumi_object_field(
                    "allowed_instance_types",
                    &self.r#allowed_instance_types,
                ),
                to_pulumi_object_field(
                    "bare_metal",
                    &self.r#bare_metal,
                ),
                to_pulumi_object_field(
                    "baseline_ebs_bandwidth_mbps",
                    &self.r#baseline_ebs_bandwidth_mbps,
                ),
                to_pulumi_object_field(
                    "burstable_performance",
                    &self.r#burstable_performance,
                ),
                to_pulumi_object_field(
                    "cpu_manufacturers",
                    &self.r#cpu_manufacturers,
                ),
                to_pulumi_object_field(
                    "excluded_instance_types",
                    &self.r#excluded_instance_types,
                ),
                to_pulumi_object_field(
                    "instance_generations",
                    &self.r#instance_generations,
                ),
                to_pulumi_object_field(
                    "local_storage",
                    &self.r#local_storage,
                ),
                to_pulumi_object_field(
                    "local_storage_types",
                    &self.r#local_storage_types,
                ),
                to_pulumi_object_field(
                    "max_spot_price_as_percentage_of_optimal_on_demand_price",
                    &self.r#max_spot_price_as_percentage_of_optimal_on_demand_price,
                ),
                to_pulumi_object_field(
                    "memory_gib_per_vcpus",
                    &self.r#memory_gib_per_vcpus,
                ),
                to_pulumi_object_field(
                    "memory_mibs",
                    &self.r#memory_mibs,
                ),
                to_pulumi_object_field(
                    "network_bandwidth_gbps",
                    &self.r#network_bandwidth_gbps,
                ),
                to_pulumi_object_field(
                    "network_interface_counts",
                    &self.r#network_interface_counts,
                ),
                to_pulumi_object_field(
                    "on_demand_max_price_percentage_over_lowest_price",
                    &self.r#on_demand_max_price_percentage_over_lowest_price,
                ),
                to_pulumi_object_field(
                    "require_hibernate_support",
                    &self.r#require_hibernate_support,
                ),
                to_pulumi_object_field(
                    "spot_max_price_percentage_over_lowest_price",
                    &self.r#spot_max_price_percentage_over_lowest_price,
                ),
                to_pulumi_object_field(
                    "total_local_storage_gbs",
                    &self.r#total_local_storage_gbs,
                ),
                to_pulumi_object_field(
                    "vcpu_counts",
                    &self.r#vcpu_counts,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirement {
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
                    r#accelerator_counts: {
                        let field_value = match fields_map.get("accelerator_counts") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_counts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#accelerator_manufacturers: {
                        let field_value = match fields_map.get("accelerator_manufacturers") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_manufacturers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#accelerator_names: {
                        let field_value = match fields_map.get("accelerator_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#accelerator_total_memory_mibs: {
                        let field_value = match fields_map.get("accelerator_total_memory_mibs") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_total_memory_mibs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#accelerator_types: {
                        let field_value = match fields_map.get("accelerator_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowed_instance_types: {
                        let field_value = match fields_map.get("allowed_instance_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_instance_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bare_metal: {
                        let field_value = match fields_map.get("bare_metal") {
                            Some(value) => value,
                            None => bail!("Missing field 'bare_metal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#baseline_ebs_bandwidth_mbps: {
                        let field_value = match fields_map.get("baseline_ebs_bandwidth_mbps") {
                            Some(value) => value,
                            None => bail!("Missing field 'baseline_ebs_bandwidth_mbps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#burstable_performance: {
                        let field_value = match fields_map.get("burstable_performance") {
                            Some(value) => value,
                            None => bail!("Missing field 'burstable_performance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_manufacturers: {
                        let field_value = match fields_map.get("cpu_manufacturers") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_manufacturers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_instance_types: {
                        let field_value = match fields_map.get("excluded_instance_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_instance_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_generations: {
                        let field_value = match fields_map.get("instance_generations") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_generations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_storage: {
                        let field_value = match fields_map.get("local_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_storage_types: {
                        let field_value = match fields_map.get("local_storage_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_storage_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_spot_price_as_percentage_of_optimal_on_demand_price: {
                        let field_value = match fields_map.get("max_spot_price_as_percentage_of_optimal_on_demand_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_spot_price_as_percentage_of_optimal_on_demand_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_gib_per_vcpus: {
                        let field_value = match fields_map.get("memory_gib_per_vcpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_gib_per_vcpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_mibs: {
                        let field_value = match fields_map.get("memory_mibs") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_mibs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_bandwidth_gbps: {
                        let field_value = match fields_map.get("network_bandwidth_gbps") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_bandwidth_gbps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interface_counts: {
                        let field_value = match fields_map.get("network_interface_counts") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_interface_counts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_demand_max_price_percentage_over_lowest_price: {
                        let field_value = match fields_map.get("on_demand_max_price_percentage_over_lowest_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_demand_max_price_percentage_over_lowest_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_hibernate_support: {
                        let field_value = match fields_map.get("require_hibernate_support") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_hibernate_support' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_max_price_percentage_over_lowest_price: {
                        let field_value = match fields_map.get("spot_max_price_percentage_over_lowest_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'spot_max_price_percentage_over_lowest_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_local_storage_gbs: {
                        let field_value = match fields_map.get("total_local_storage_gbs") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_local_storage_gbs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vcpu_counts: {
                        let field_value = match fields_map.get("vcpu_counts") {
                            Some(value) => value,
                            None => bail!("Missing field 'vcpu_counts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
