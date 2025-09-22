#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLaunchTemplateInstanceRequirement {
    #[builder(into)]
    #[serde(rename = "acceleratorCounts")]
    pub r#accelerator_counts: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementAcceleratorCount>,
    #[builder(into)]
    #[serde(rename = "acceleratorManufacturers")]
    pub r#accelerator_manufacturers: Vec<String>,
    #[builder(into)]
    #[serde(rename = "acceleratorNames")]
    pub r#accelerator_names: Vec<String>,
    #[builder(into)]
    #[serde(rename = "acceleratorTotalMemoryMibs")]
    pub r#accelerator_total_memory_mibs: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementAcceleratorTotalMemoryMib>,
    #[builder(into)]
    #[serde(rename = "acceleratorTypes")]
    pub r#accelerator_types: Vec<String>,
    #[builder(into)]
    #[serde(rename = "allowedInstanceTypes")]
    pub r#allowed_instance_types: Vec<String>,
    #[builder(into)]
    #[serde(rename = "bareMetal")]
    pub r#bare_metal: String,
    #[builder(into)]
    #[serde(rename = "baselineEbsBandwidthMbps")]
    pub r#baseline_ebs_bandwidth_mbps: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementBaselineEbsBandwidthMbp>,
    #[builder(into)]
    #[serde(rename = "burstablePerformance")]
    pub r#burstable_performance: String,
    #[builder(into)]
    #[serde(rename = "cpuManufacturers")]
    pub r#cpu_manufacturers: Vec<String>,
    #[builder(into)]
    #[serde(rename = "excludedInstanceTypes")]
    pub r#excluded_instance_types: Vec<String>,
    #[builder(into)]
    #[serde(rename = "instanceGenerations")]
    pub r#instance_generations: Vec<String>,
    #[builder(into)]
    #[serde(rename = "localStorage")]
    pub r#local_storage: String,
    #[builder(into)]
    #[serde(rename = "localStorageTypes")]
    pub r#local_storage_types: Vec<String>,
    #[builder(into)]
    #[serde(rename = "maxSpotPriceAsPercentageOfOptimalOnDemandPrice")]
    pub r#max_spot_price_as_percentage_of_optimal_on_demand_price: i32,
    #[builder(into)]
    #[serde(rename = "memoryGibPerVcpus")]
    pub r#memory_gib_per_vcpus: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementMemoryGibPerVcpus>,
    #[builder(into)]
    #[serde(rename = "memoryMibs")]
    pub r#memory_mibs: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementMemoryMib>,
    #[builder(into)]
    #[serde(rename = "networkBandwidthGbps")]
    pub r#network_bandwidth_gbps: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementNetworkBandwidthGbp>,
    #[builder(into)]
    #[serde(rename = "networkInterfaceCounts")]
    pub r#network_interface_counts: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementNetworkInterfaceCount>,
    #[builder(into)]
    #[serde(rename = "onDemandMaxPricePercentageOverLowestPrice")]
    pub r#on_demand_max_price_percentage_over_lowest_price: i32,
    #[builder(into)]
    #[serde(rename = "requireHibernateSupport")]
    pub r#require_hibernate_support: bool,
    #[builder(into)]
    #[serde(rename = "spotMaxPricePercentageOverLowestPrice")]
    pub r#spot_max_price_percentage_over_lowest_price: i32,
    #[builder(into)]
    #[serde(rename = "totalLocalStorageGbs")]
    pub r#total_local_storage_gbs: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementTotalLocalStorageGb>,
    #[builder(into)]
    #[serde(rename = "vcpuCounts")]
    pub r#vcpu_counts: Vec<super::super::types::ec2::GetLaunchTemplateInstanceRequirementVcpuCount>,
}
