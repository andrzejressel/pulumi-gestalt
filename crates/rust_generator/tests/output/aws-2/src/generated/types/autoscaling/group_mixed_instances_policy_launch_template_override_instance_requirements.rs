#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirements {
    /// Block describing the minimum and maximum number of accelerators (GPUs, FPGAs, or AWS Inferentia chips). Default is no minimum or maximum.
    #[builder(into)]
    #[serde(rename = "acceleratorCount")]
    pub r#accelerator_count: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsAcceleratorCount>>,
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
    pub r#accelerator_total_memory_mib: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsAcceleratorTotalMemoryMib>>,
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
    pub r#baseline_ebs_bandwidth_mbps: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsBaselineEbsBandwidthMbps>>,
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
    pub r#memory_gib_per_vcpu: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsMemoryGibPerVcpu>>,
    /// Block describing the minimum and maximum amount of memory (MiB). Default is no maximum.
    #[builder(into)]
    #[serde(rename = "memoryMib")]
    pub r#memory_mib: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsMemoryMib>>,
    /// Block describing the minimum and maximum amount of network bandwidth, in gigabits per second (Gbps). Default is no minimum or maximum.
    #[builder(into)]
    #[serde(rename = "networkBandwidthGbps")]
    pub r#network_bandwidth_gbps: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsNetworkBandwidthGbps>>,
    /// Block describing the minimum and maximum number of network interfaces. Default is no minimum or maximum.
    #[builder(into)]
    #[serde(rename = "networkInterfaceCount")]
    pub r#network_interface_count: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsNetworkInterfaceCount>>,
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
    pub r#total_local_storage_gb: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsTotalLocalStorageGb>>,
    /// Block describing the minimum and maximum number of vCPUs. Default is no maximum.
    #[builder(into)]
    #[serde(rename = "vcpuCount")]
    pub r#vcpu_count: Box<Option<super::super::types::autoscaling::GroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementsVcpuCount>>,
}
