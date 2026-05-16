#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirements {
    /// Block describing the minimum and maximum number of accelerators (GPUs, FPGAs, or AWS Inferentia chips). Default is no minimum or maximum.
    #[builder(into)]
    #[serde(rename = "acceleratorCount")]
    pub r#accelerator_count: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsAcceleratorCount>>,
    /// List of accelerator manufacturer names. Default is any manufacturer.
    /// 
    /// ```sh
    /// Valid names:
    /// * amazon-web-services
    /// * amd
    /// * nvidia
    /// * xilinx
    /// ```
    #[builder(into)]
    #[serde(rename = "acceleratorManufacturers")]
    pub r#accelerator_manufacturers: Option<Vec<String>>,
    /// List of accelerator names. Default is any acclerator.
    /// 
    /// ```sh
    /// Valid names:
    /// * a100            - NVIDIA A100 GPUs
    /// * v100            - NVIDIA V100 GPUs
    /// * k80             - NVIDIA K80 GPUs
    /// * t4              - NVIDIA T4 GPUs
    /// * m60             - NVIDIA M60 GPUs
    /// * radeon-pro-v520 - AMD Radeon Pro V520 GPUs
    /// * vu9p            - Xilinx VU9P FPGAs
    /// ```
    #[builder(into)]
    #[serde(rename = "acceleratorNames")]
    pub r#accelerator_names: Option<Vec<String>>,
    /// Block describing the minimum and maximum total memory of the accelerators. Default is no minimum or maximum.
    #[builder(into)]
    #[serde(rename = "acceleratorTotalMemoryMib")]
    pub r#accelerator_total_memory_mib: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsAcceleratorTotalMemoryMib>>,
    /// List of accelerator types. Default is any accelerator type.
    /// 
    /// ```sh
    /// Valid types:
    /// * fpga
    /// * gpu
    /// * inference
    /// ```
    #[builder(into)]
    #[serde(rename = "acceleratorTypes")]
    pub r#accelerator_types: Option<Vec<String>>,
    /// List of instance types to apply your specified attributes against. All other instance types are ignored, even if they match your specified attributes. You can use strings with one or more wild cards, represented by an asterisk (\*), to allow an instance type, size, or generation. The following are examples: `m5.8xlarge`, `c5*.*`, `m5a.*`, `r*`, `*3*`. For example, if you specify `c5*`, you are allowing the entire C5 instance family, which includes all C5a and C5n instance types. If you specify `m5a.*`, you are allowing all the M5a instance types, but not the M5n instance types. Maximum of 400 entries in the list; each entry is limited to 30 characters. Default is all instance types.
    /// 
    /// > **NOTE:** If you specify `allowed_instance_types`, you can't specify `excluded_instance_types`.
    #[builder(into)]
    #[serde(rename = "allowedInstanceTypes")]
    pub r#allowed_instance_types: Option<Vec<String>>,
    /// Indicate whether bare metal instace types should be `included`, `excluded`, or `required`. Default is `excluded`.
    #[builder(into)]
    #[serde(rename = "bareMetal")]
    pub r#bare_metal: Option<String>,
    /// Block describing the minimum and maximum baseline EBS bandwidth, in Mbps. Default is no minimum or maximum.
    #[builder(into)]
    #[serde(rename = "baselineEbsBandwidthMbps")]
    pub r#baseline_ebs_bandwidth_mbps: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsBaselineEbsBandwidthMbps>>,
    /// Indicate whether burstable performance instance types should be `included`, `excluded`, or `required`. Default is `excluded`.
    #[builder(into)]
    #[serde(rename = "burstablePerformance")]
    pub r#burstable_performance: Option<String>,
    /// List of CPU manufacturer names. Default is any manufacturer.
    /// 
    /// > **NOTE:** Don't confuse the CPU hardware manufacturer with the CPU hardware architecture. Instances will be launched with a compatible CPU architecture based on the Amazon Machine Image (AMI) that you specify in your launch template.
    /// 
    /// ```sh
    /// Valid names:
    /// * amazon-web-services
    /// * amd
    /// * intel
    /// ```
    #[builder(into)]
    #[serde(rename = "cpuManufacturers")]
    pub r#cpu_manufacturers: Option<Vec<String>>,
    /// List of instance types to exclude. You can use strings with one or more wild cards, represented by an asterisk (\*), to exclude an instance type, size, or generation. The following are examples: `m5.8xlarge`, `c5*.*`, `m5a.*`, `r*`, `*3*`. For example, if you specify `c5*`, you are excluding the entire C5 instance family, which includes all C5a and C5n instance types. If you specify `m5a.*`, you are excluding all the M5a instance types, but not the M5n instance types. Maximum of 400 entries in the list; each entry is limited to 30 characters. Default is no excluded instance types.
    /// 
    /// > **NOTE:** If you specify `excluded_instance_types`, you can't specify `allowed_instance_types`.
    #[builder(into)]
    #[serde(rename = "excludedInstanceTypes")]
    pub r#excluded_instance_types: Option<Vec<String>>,
    /// List of instance generation names. Default is any generation.
    /// 
    /// ```sh
    /// Valid names:
    /// * current  - Recommended for best performance.
    /// * previous - For existing applications optimized for older instance types.
    /// ```
    #[builder(into)]
    #[serde(rename = "instanceGenerations")]
    pub r#instance_generations: Option<Vec<String>>,
    /// Indicate whether instance types with local storage volumes are `included`, `excluded`, or `required`. Default is `included`.
    #[builder(into)]
    #[serde(rename = "localStorage")]
    pub r#local_storage: Option<String>,
    /// List of local storage type names. Default any storage type.
    /// 
    /// ```sh
    /// Value names:
    /// * hdd - hard disk drive
    /// * ssd - solid state drive
    /// ```
    #[builder(into)]
    #[serde(rename = "localStorageTypes")]
    pub r#local_storage_types: Option<Vec<String>>,
    /// The price protection threshold for Spot Instances. This is the maximum you’ll pay for a Spot Instance, expressed as a percentage higher than the cheapest M, C, or R instance type with your specified attributes. When Amazon EC2 Auto Scaling selects instance types with your attributes, we will exclude instance types whose price is higher than your threshold. The parameter accepts an integer, which Amazon EC2 Auto Scaling interprets as a percentage. To turn off price protection, specify a high value, such as 999999. Conflicts with `spot_max_price_percentage_over_lowest_price`
    #[builder(into)]
    #[serde(rename = "maxSpotPriceAsPercentageOfOptimalOnDemandPrice")]
    pub r#max_spot_price_as_percentage_of_optimal_on_demand_price: Option<i32>,
    /// Block describing the minimum and maximum amount of memory (GiB) per vCPU. Default is no minimum or maximum.
    #[builder(into)]
    #[serde(rename = "memoryGibPerVcpu")]
    pub r#memory_gib_per_vcpu: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsMemoryGibPerVcpu>>,
    /// Block describing the minimum and maximum amount of memory (MiB). Default is no maximum.
    #[builder(into)]
    #[serde(rename = "memoryMib")]
    pub r#memory_mib: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsMemoryMib>>,
    /// Block describing the minimum and maximum amount of network bandwidth, in gigabits per second (Gbps). Default is no minimum or maximum.
    #[builder(into)]
    #[serde(rename = "networkBandwidthGbps")]
    pub r#network_bandwidth_gbps: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsNetworkBandwidthGbps>>,
    /// Block describing the minimum and maximum number of network interfaces. Default is no minimum or maximum.
    #[builder(into)]
    #[serde(rename = "networkInterfaceCount")]
    pub r#network_interface_count: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsNetworkInterfaceCount>>,
    /// Price protection threshold for On-Demand Instances. This is the maximum you’ll pay for an On-Demand Instance, expressed as a percentage higher than the cheapest M, C, or R instance type with your specified attributes. When Amazon EC2 Auto Scaling selects instance types with your attributes, we will exclude instance types whose price is higher than your threshold. The parameter accepts an integer, which Amazon EC2 Auto Scaling interprets as a percentage. To turn off price protection, specify a high value, such as 999999. Default is 20.
    /// 
    /// If you set DesiredCapacityType to vcpu or memory-mib, the price protection threshold is applied based on the per vCPU or per memory price instead of the per instance price.
    #[builder(into)]
    #[serde(rename = "onDemandMaxPricePercentageOverLowestPrice")]
    pub r#on_demand_max_price_percentage_over_lowest_price: Option<i32>,
    /// Indicate whether instance types must support On-Demand Instance Hibernation, either `true` or `false`. Default is `false`.
    #[builder(into)]
    #[serde(rename = "requireHibernateSupport")]
    pub r#require_hibernate_support: Option<bool>,
    /// Price protection threshold for Spot Instances. This is the maximum you’ll pay for a Spot Instance, expressed as a percentage higher than the cheapest M, C, or R instance type with your specified attributes. When Amazon EC2 Auto Scaling selects instance types with your attributes, we will exclude instance types whose price is higher than your threshold. The parameter accepts an integer, which Amazon EC2 Auto Scaling interprets as a percentage. To turn off price protection, specify a high value, such as 999999. Default is 100. Conflicts with `max_spot_price_as_percentage_of_optimal_on_demand_price`
    /// 
    /// If you set DesiredCapacityType to vcpu or memory-mib, the price protection threshold is applied based on the per vCPU or per memory price instead of the per instance price.
    #[builder(into)]
    #[serde(rename = "spotMaxPricePercentageOverLowestPrice")]
    pub r#spot_max_price_percentage_over_lowest_price: Option<i32>,
    /// Block describing the minimum and maximum total local storage (GB). Default is no minimum or maximum.
    #[builder(into)]
    #[serde(rename = "totalLocalStorageGb")]
    pub r#total_local_storage_gb: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsTotalLocalStorageGb>>,
    /// Block describing the minimum and maximum number of vCPUs. Default is no maximum.
    #[builder(into)]
    #[serde(rename = "vcpuCount")]
    pub r#vcpu_count: Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsVcpuCount>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirements {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("accelerator_count".to_string(), self.r#accelerator_count.to_pulumi_value().await);
            map.insert("accelerator_manufacturers".to_string(), self.r#accelerator_manufacturers.to_pulumi_value().await);
            map.insert("accelerator_names".to_string(), self.r#accelerator_names.to_pulumi_value().await);
            map.insert("accelerator_total_memory_mib".to_string(), self.r#accelerator_total_memory_mib.to_pulumi_value().await);
            map.insert("accelerator_types".to_string(), self.r#accelerator_types.to_pulumi_value().await);
            map.insert("allowed_instance_types".to_string(), self.r#allowed_instance_types.to_pulumi_value().await);
            map.insert("bare_metal".to_string(), self.r#bare_metal.to_pulumi_value().await);
            map.insert("baseline_ebs_bandwidth_mbps".to_string(), self.r#baseline_ebs_bandwidth_mbps.to_pulumi_value().await);
            map.insert("burstable_performance".to_string(), self.r#burstable_performance.to_pulumi_value().await);
            map.insert("cpu_manufacturers".to_string(), self.r#cpu_manufacturers.to_pulumi_value().await);
            map.insert("excluded_instance_types".to_string(), self.r#excluded_instance_types.to_pulumi_value().await);
            map.insert("instance_generations".to_string(), self.r#instance_generations.to_pulumi_value().await);
            map.insert("local_storage".to_string(), self.r#local_storage.to_pulumi_value().await);
            map.insert("local_storage_types".to_string(), self.r#local_storage_types.to_pulumi_value().await);
            map.insert("max_spot_price_as_percentage_of_optimal_on_demand_price".to_string(), self.r#max_spot_price_as_percentage_of_optimal_on_demand_price.to_pulumi_value().await);
            map.insert("memory_gib_per_vcpu".to_string(), self.r#memory_gib_per_vcpu.to_pulumi_value().await);
            map.insert("memory_mib".to_string(), self.r#memory_mib.to_pulumi_value().await);
            map.insert("network_bandwidth_gbps".to_string(), self.r#network_bandwidth_gbps.to_pulumi_value().await);
            map.insert("network_interface_count".to_string(), self.r#network_interface_count.to_pulumi_value().await);
            map.insert("on_demand_max_price_percentage_over_lowest_price".to_string(), self.r#on_demand_max_price_percentage_over_lowest_price.to_pulumi_value().await);
            map.insert("require_hibernate_support".to_string(), self.r#require_hibernate_support.to_pulumi_value().await);
            map.insert("spot_max_price_percentage_over_lowest_price".to_string(), self.r#spot_max_price_percentage_over_lowest_price.to_pulumi_value().await);
            map.insert("total_local_storage_gb".to_string(), self.r#total_local_storage_gb.to_pulumi_value().await);
            map.insert("vcpu_count".to_string(), self.r#vcpu_count.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirements {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#accelerator_count: {
                        let field_value = match fields_map.get("accelerator_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsAcceleratorCount>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#accelerator_manufacturers: {
                        let field_value = match fields_map.get("accelerator_manufacturers") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_manufacturers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#accelerator_names: {
                        let field_value = match fields_map.get("accelerator_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#accelerator_total_memory_mib: {
                        let field_value = match fields_map.get("accelerator_total_memory_mib") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_total_memory_mib' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsAcceleratorTotalMemoryMib>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#accelerator_types: {
                        let field_value = match fields_map.get("accelerator_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#allowed_instance_types: {
                        let field_value = match fields_map.get("allowed_instance_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_instance_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#bare_metal: {
                        let field_value = match fields_map.get("bare_metal") {
                            Some(value) => value,
                            None => bail!("Missing field 'bare_metal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#baseline_ebs_bandwidth_mbps: {
                        let field_value = match fields_map.get("baseline_ebs_bandwidth_mbps") {
                            Some(value) => value,
                            None => bail!("Missing field 'baseline_ebs_bandwidth_mbps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsBaselineEbsBandwidthMbps>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#burstable_performance: {
                        let field_value = match fields_map.get("burstable_performance") {
                            Some(value) => value,
                            None => bail!("Missing field 'burstable_performance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cpu_manufacturers: {
                        let field_value = match fields_map.get("cpu_manufacturers") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_manufacturers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#excluded_instance_types: {
                        let field_value = match fields_map.get("excluded_instance_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_instance_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#instance_generations: {
                        let field_value = match fields_map.get("instance_generations") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_generations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#local_storage: {
                        let field_value = match fields_map.get("local_storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#local_storage_types: {
                        let field_value = match fields_map.get("local_storage_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_storage_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_spot_price_as_percentage_of_optimal_on_demand_price: {
                        let field_value = match fields_map.get("max_spot_price_as_percentage_of_optimal_on_demand_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_spot_price_as_percentage_of_optimal_on_demand_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#memory_gib_per_vcpu: {
                        let field_value = match fields_map.get("memory_gib_per_vcpu") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_gib_per_vcpu' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsMemoryGibPerVcpu>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#memory_mib: {
                        let field_value = match fields_map.get("memory_mib") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_mib' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsMemoryMib>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#network_bandwidth_gbps: {
                        let field_value = match fields_map.get("network_bandwidth_gbps") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_bandwidth_gbps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsNetworkBandwidthGbps>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#network_interface_count: {
                        let field_value = match fields_map.get("network_interface_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_interface_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsNetworkInterfaceCount>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#on_demand_max_price_percentage_over_lowest_price: {
                        let field_value = match fields_map.get("on_demand_max_price_percentage_over_lowest_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_demand_max_price_percentage_over_lowest_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#require_hibernate_support: {
                        let field_value = match fields_map.get("require_hibernate_support") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_hibernate_support' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#spot_max_price_percentage_over_lowest_price: {
                        let field_value = match fields_map.get("spot_max_price_percentage_over_lowest_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'spot_max_price_percentage_over_lowest_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#total_local_storage_gb: {
                        let field_value = match fields_map.get("total_local_storage_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_local_storage_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsTotalLocalStorageGb>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vcpu_count: {
                        let field_value = match fields_map.get("vcpu_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'vcpu_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsVcpuCount>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
